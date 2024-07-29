use axum::extract::{FromRef, State};
use axum::http::header::USER_AGENT;
use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts,
    RequestPartsExt,
};
use axum_extra::extract::CookieJar;
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;

use super::{logic, models, sql};
use crate::errors::Error;
use crate::state::AppState;

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

pub(crate) struct UserAgent(pub(crate) String);
#[async_trait]
impl<S> FromRequestParts<S> for UserAgent
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            Ok(UserAgent(
                user_agent
                    .to_str()
                    .map_err(|err| {
                        tracing::error!("Failed to parse user agent: {err}");
                        Error::MalformedRequest("Unable to parse user agent")
                    })?
                    .to_string(),
            ))
        } else {
            Err(Error::MalformedRequest("Missing user agent"))
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for models::Claims
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            let token = bearer.token();
            if token.starts_with("manag.") {
                let claims = logic::decode_management_claims(token)?;

                let state = parts
                    .extensions
                    .get::<State<AppState>>()
                    .ok_or(Error::IllegalState)?
                    .clone();

                let permissions = sql::fetch_management_token_permissions(
                    &state.pool,
                    claims.jti,
                )
                .await?
                .ok_or(Error::Unauthorized)?;

                if permissions.revoked {
                    return Err(Error::Unauthorized);
                }

                return Ok(models::Claims {
                    exp: claims.exp,
                    iat: claims.iat,
                    nbf: 0,
                    jti: claims.jti,
                    origin: claims.jti,
                    uid: claims.uid,
                    permissions: permissions.permissions.0,
                });
            }
            return logic::decode_access_claims(bearer.token());
        }

        if let Ok(jar) = parts.extract::<CookieJar>().await {
            if let Some(cookie) = jar.get("access_token") {
                let value = cookie.value();
                return logic::decode_access_claims(value);
            }
        }

        Err(Error::Unauthorized)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for models::RefreshClaims
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            return logic::decode_refresh_claims(bearer.token());
        }

        if let Ok(jar) = parts.extract::<CookieJar>().await {
            if let Some(cookie) = jar.get("refresh_token") {
                let value = cookie.value();
                return logic::decode_refresh_claims(value);
            }
        }

        Err(Error::Unauthorized)
    }
}

#[async_trait]
impl<S, P: super::ClaimPermission> FromRequestParts<S> for super::ScopedClaim<P>
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let claims = models::Claims::from_request_parts(parts, state).await?;
        if P::is_valid(&claims.permissions) {
            Ok(Self(claims, std::marker::PhantomData))
        } else {
            Err(Error::Forbidden)
        }
    }
}
