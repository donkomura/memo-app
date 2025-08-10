use actix_web::{FromRequest, HttpRequest, dev::Payload, http::header, web};
use std::future::{Ready, ready};

use super::{model::JWTClaim, token::JwtTokenService};

pub struct AuthenticatedUser(pub JWTClaim);

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt = req.app_data::<web::Data<JwtTokenService>>();
        let auth = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let Some(jwt) = jwt else {
            return ready(Err(actix_web::error::ErrorUnauthorized("missing jwt")));
        };
        let Some(auth) = auth else {
            return ready(Err(actix_web::error::ErrorUnauthorized("missing header")));
        };

        let token = auth.strip_prefix("Bearer ").unwrap_or("");
        if token.is_empty() {
            return ready(Err(actix_web::error::ErrorUnauthorized("invalid header")));
        }

        match jwt.verify(token) {
            Ok(claim) => ready(Ok(AuthenticatedUser(claim))),
            Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("invalid token"))),
        }
    }
}
