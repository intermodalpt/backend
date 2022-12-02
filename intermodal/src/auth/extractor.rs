use axum::headers::{authorization::Bearer, Authorization};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
};

use super::{logic, models};
use crate::errors::Error;

#[async_trait]
impl<B> FromRequest<B> for models::Claims
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(
        req: &mut RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|err| Error::DependenciesNotMet)?;

        logic::decode_claims(bearer.token())
    }
}
