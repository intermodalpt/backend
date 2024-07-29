use crate::auth::{
    models, ACCESS_SECRET_KEY, MANAGEMENT_SECRET_KEY, REFRESH_SECRET_KEY,
};
use crate::Error;

pub(crate) fn encode_access_claims(
    claims: &models::Claims,
) -> Result<models::JwtAccess, Error> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    let encoding_key = jsonwebtoken::EncodingKey::from_secret(
        ACCESS_SECRET_KEY.get().unwrap().as_ref(),
    );
    jsonwebtoken::encode(&header, claims, &encoding_key)
        .map(models::JwtAccess)
        .map_err(|err| {
            tracing::error!("Failed to encode Access JWT: {err}");
            Error::Processing
        })
}

pub(crate) fn decode_access_claims(jwt: &str) -> Result<models::Claims, Error> {
    let mut validation =
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_nbf = true;
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(
        ACCESS_SECRET_KEY.get().unwrap().as_ref(),
    );
    let decoded_token =
        jsonwebtoken::decode::<models::Claims>(jwt, &decoding_key, &validation)
            .map_err(|err| {
                tracing::error!("Failed to decode Access JWT: {err}");
                Error::Forbidden
            })?;

    Ok(decoded_token.claims)
}

pub(crate) fn encode_refresh_claims(
    claims: &models::RefreshClaims,
) -> Result<models::JwtRefresh, Error> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    jsonwebtoken::encode(
        &header,
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            REFRESH_SECRET_KEY.get().unwrap().as_ref(),
        ),
    )
    .map(models::JwtRefresh)
    .map_err(|err| {
        tracing::error!("Failed to encode Refresh JWT: {err}");
        Error::Processing
    })
}

pub(crate) fn decode_refresh_claims(
    jwt: &str,
) -> Result<models::RefreshClaims, Error> {
    let mut validation =
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_nbf = true;
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(
        REFRESH_SECRET_KEY.get().unwrap().as_ref(),
    );
    let decoded_token = jsonwebtoken::decode::<models::RefreshClaims>(
        jwt,
        &decoding_key,
        &validation,
    )
    .map_err(|err| {
        tracing::error!("Failed to decode Refresh JWT: {err}");
        Error::Forbidden
    })?;

    Ok(decoded_token.claims)
}

pub(crate) fn encode_management_claims(
    claims: &models::ManagementClaims,
) -> Result<models::JwtManagement, Error> {
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
    jsonwebtoken::encode(
        &header,
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            MANAGEMENT_SECRET_KEY.get().unwrap().as_ref(),
        ),
    )
    .map(|token| models::JwtManagement(format!("manag.{token}")))
    .map_err(|err| {
        tracing::error!("Failed to encode Management JWT: {err}");
        Error::Processing
    })
}

pub(crate) fn decode_management_claims(
    jwt: &str,
) -> Result<models::ManagementClaims, Error> {
    let validation =
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(
        MANAGEMENT_SECRET_KEY.get().unwrap().as_ref(),
    );
    let decoded_token = jsonwebtoken::decode::<models::ManagementClaims>(
        &jwt[6..],
        &decoding_key,
        &validation,
    )
    .map_err(|err| {
        tracing::error!("Failed to decode Management JWT: {err}");
        Error::Forbidden
    })?;

    Ok(decoded_token.claims)
}

#[cfg(test)]
mod tests {
    use commons::models::auth::Permissions;

    use crate::auth::{models, models::requests};
    use crate::errors::Error;

    #[test]
    fn encode_decode_refresh_claims() {
        use super::*;

        //The key must be set
        let _ = REFRESH_SECRET_KEY
            .set(Box::leak(Box::new("super_secret_key".to_string())));

        let claims = models::RefreshClaims {
            iat: 0,
            exp: 99999999999,
            uid: 123,
            jti: Default::default(),
            uname: "foo".to_string(),
        };
        let encoded = encode_refresh_claims(&claims).unwrap();
        let decoded = decode_refresh_claims(&encoded.0).unwrap();
        assert_eq!(claims, decoded);
    }

    #[test]
    fn encode_decode_access_claims() {
        use super::*;

        //The key must be set
        let _ = ACCESS_SECRET_KEY
            .set(Box::leak(Box::new("super_secret_key".to_string())));

        let claims = models::Claims {
            iat: 0,
            nbf: 0,
            jti: Default::default(),
            exp: 99999999999,
            uid: 0,
            permissions: Permissions::default(),
            origin: Default::default(),
        };
        let encoded = encode_access_claims(&claims).unwrap();
        let decoded = decode_access_claims(&encoded.0).unwrap();
        assert_eq!(claims, decoded);
    }

    #[test]
    fn encode_decode_management_claims() {
        use super::*;

        //The key must be set
        let _ = MANAGEMENT_SECRET_KEY
            .set(Box::leak(Box::new("super_secret_key".to_string())));
        let claims = models::ManagementClaims {
            iat: 0,
            exp: 99999999999,
            uid: 123,
            jti: Default::default(),
        };
        let encoded = encode_management_claims(&claims).unwrap();
        let decoded = decode_management_claims(&encoded.0).unwrap();
        assert_eq!(claims, decoded);
    }
}
