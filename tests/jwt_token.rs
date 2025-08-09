use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use memo_app::middleware::auth::model::JWTClaim;
use memo_app::middleware::auth::token::{JwtTokenService, TokenError};

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[test]
fn generate_and_verify_returns_claim() {
    let secret = b"secret";
    let svc = JwtTokenService::from_secret(secret, 3600);

    let token = svc.generate(42).expect("token");
    let claim = svc.verify(&token).expect("verify");

    assert_eq!(claim.sub, 42);
    assert!(claim.exp > claim.iat);
}

#[test]
fn verify_fails_when_expired() {
    let secret = b"secret";
    let svc = JwtTokenService::from_secret(secret, 3600);

    // exp を過去にしたトークンを手動生成
    let past = now() - 1;
    let claim = JWTClaim { sub: 1, iat: past - 10, exp: past };
    let token = encode(&Header::new(Algorithm::HS256), &claim, &EncodingKey::from_secret(secret))
        .expect("encode");

    let res = svc.verify(&token);
    assert!(matches!(res, Err(TokenError::Decode)));
}

#[test]
fn verify_fails_with_wrong_secret() {
    let good = JwtTokenService::from_secret(b"good", 3600);
    let bad = JwtTokenService::from_secret(b"bad", 3600);

    let token = good.generate(7).expect("token");
    let res = bad.verify(&token);
    assert!(matches!(res, Err(TokenError::Decode)));
}


