use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JWTClaim {
    pub sub: i64, // user id
    pub iat: i64, // issued at
    pub exp: i64, // expire
}


