use chrono::{Duration, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

static PASETO_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    std::env::var("PASETO_KEY")
        .unwrap_or_else(|_| panic!("missing PASETO_KEY environment variable"))
        .as_bytes()
        .to_owned()
});

/// Creates a download token for a specific user id (email)
pub(crate) fn create_download_token(user_id: &str, signup_email: &str) -> String {
    let current_dt = Utc::now();
    let dt = current_dt
        .checked_add_signed(Duration::minutes(30))
        .unwrap();

    match paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&PASETO_KEY)
        .set_expiration(&dt)
        .set_issuer("SCP")
        .set_not_before(&Utc::now())
        .set_claim("user_id", json!(user_id))
        .set_claim("signup_email", json!(signup_email))
        .build()
    {
        Ok(token) => token,
        Err(e) => {
            error!("e = {:#?}", e);
            panic!()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PasetoResult {
    pub iss: String,
    /// Not Before
    pub nbf: String,
    pub user_id: String,
    /// The email that was used to generate this token.
    pub signup_email: String,
    /// Expiry
    pub exp: String,
}

/// Decrypts a download token and returns the ID from within the token.
pub(crate) fn decrypt_download_token(token: &str) -> Option<PasetoResult> {
    let decrypted = match paseto::v2::decrypt_paseto(token, None, &PASETO_KEY) {
        Ok(t) => t,
        Err(e) => {
            error!("e = {:#?}", e);
            return None;
        }
    };

    let result = match serde_json::from_str::<PasetoResult>(&decrypted) {
        Ok(value) => value,
        Err(e) => {
            error!("e = {:#?}", e);
            return None;
        }
    };

    let valid = match chrono::DateTime::parse_from_rfc3339(&result.exp) {
        Ok(time) => time > Utc::now(),
        Err(e) => {
            error!("e = {:#?}", e);
            return None;
        }
    };

    if !valid {
        error!("past expiry date");
        return None;
    }

    // It is valid
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates() {
        let t = create_download_token("_scpUz1234567@unsw.scp.platform", "some@email.com");
        error!("t = {:#?}", t);
    }
}
