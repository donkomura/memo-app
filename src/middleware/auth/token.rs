use std::time::{SystemTime, UNIX_EPOCH, Duration};

use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use thiserror::Error;

use super::model::JWTClaim;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("encoding error")]
    Encode,
    #[error("decoding error")]
    Decode,
    #[error("missing JWT_SECRET env")] 
    MissingSecret,
    #[error("invalid JWT_EXP_SECS env")] 
    InvalidExpiration,
}

pub struct JwtTokenService {
    encoding: EncodingKey,
    decoding: DecodingKey,
    expiration_secs: u64,
}

impl JwtTokenService {
    const DEFAULT_EXPIRATION_SECS: u64 = 3600;
    pub fn from_secret(secret: &[u8], expiration_secs: u64) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
            expiration_secs,
        }
    }

    pub fn from_env() -> Result<Self, TokenError> {
        let secret = std::env::var("JWT_SECRET").map_err(|_| TokenError::MissingSecret)?;
        let exp = match std::env::var("JWT_EXP_SECS") {
            Ok(v) => v.parse::<u64>().map_err(|_| TokenError::InvalidExpiration)?,
            Err(_) => Self::DEFAULT_EXPIRATION_SECS,
        };
        Ok(Self::from_secret(secret.as_bytes(), exp))
    }

    pub fn generate(&self, user_id: i64) -> Result<String, TokenError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs() as i64;
        let claims = JWTClaim {
            sub: user_id,
            iat: now,
            exp: now + self.expiration_secs as i64,
        };
        let header = Header::default();
        encode(&header, &claims, &self.encoding).map_err(|_| TokenError::Encode)
    }

    pub fn verify(&self, token: &str) -> Result<JWTClaim, TokenError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        let claims = decode::<JWTClaim>(token, &self.decoding, &validation)
            .map(|data| data.claims)
            .map_err(|_| TokenError::Decode)?;

        // 念のため手動でも exp を検証（バージョン差異対策）
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs() as i64;
        if claims.exp < now { return Err(TokenError::Decode); }
        Ok(claims)
    }
}


