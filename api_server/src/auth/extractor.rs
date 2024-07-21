use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts,
    RequestPartsExt,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;

use super::{logic, models};
use crate::errors::Error;

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
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Forbidden)?;

        logic::decode_access_claims(bearer.token())
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
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Forbidden)?;

        logic::decode_refresh_claims(bearer.token())
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
