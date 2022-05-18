use anyhow::Context;
use jwt_simple::prelude::{Claims, Duration, Ed25519KeyPair, EdDSAKeyPairLike, EdDSAPublicKeyLike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExtraClaimsData {
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct ClaimsData {
    pub username: String,
    pub user_id: String,
}

impl ExtraClaimsData {
    fn new(username: String) -> Self {
        Self { username }
    }
}

/// Create a JWT for a user to be appended for intra service communication headers. Tokens are valid for 60s.
pub fn create_jwt(user_id: String, username: String, key_pair_pem: &str) -> anyhow::Result<String> {
    let additional = ExtraClaimsData::new(username);

    let key_pair = Ed25519KeyPair::from_pem(key_pair_pem)?;

    let claims = Claims::with_custom_claims(additional, Duration::from_secs(60))
        .with_issuer("scp")
        .with_subject(user_id);

    key_pair.sign(claims)
}

/// Verify if a jwt is valid and return the claims contained within.
pub fn verify_jwt(token: &str, key_pair_pem: &str) -> anyhow::Result<ClaimsData> {
    let key_pair = Ed25519KeyPair::from_pem(key_pair_pem)?;
    let public_key = key_pair.public_key();

    let claims = public_key.verify_token::<ExtraClaimsData>(token, None)?;

    Ok(ClaimsData {
        username: claims.custom.username,
        user_id: claims.subject.context("claims did not have subject")?,
    })
}
