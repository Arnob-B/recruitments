use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, Validation};
use super::models::{Claims, AuthConfig};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let auth_config = req.app_data::<AuthConfig>()
        .expect("AuthConfig not configured");
    
    let token = credentials.token();
    let _token_data = decode::<Claims>(
        token,
        &auth_config.decoding_key,
        &Validation::default(),
    )?;

    Ok(req)
}