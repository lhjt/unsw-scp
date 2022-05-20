use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::errors::handle_error;

static PASETO_KEY: &[u8] = b"13a5e76d68e04909bc18d00646c0e34b";

pub(crate) fn create_download_token(user_id: &str) -> String {
    let current_dt = Utc::now();
    let dt = current_dt
        .checked_add_signed(Duration::minutes(15))
        .unwrap();

    match paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(PASETO_KEY)
        .set_expiration(&dt)
        .set_issuer("SCP")
        .set_not_before(&Utc::now())
        .set_claim("user_id", json!(user_id))
        .build()
    {
        Ok(token) => token,
        Err(e) => {
            eprintln!("e = {:#?}", e);
            handle_error(e.compat());
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates() {
        let t = create_download_token("z5420301");
        eprintln!("t = {:#?}", t);
    }

    #[test]
    fn it_decrypts() {
        let t = "v2.local.2_pFheSpzM0fVgV5qQAAfv0gBA__ts4E088Ts6KQQN4dqhQiJDzrun7UJr6yhG2sJ3xkTAN6xqNYX04CD8NCxDl8g98GqqWAxd7aEaqkrQacPFfE2CGrBdJLEPV6h01OT2auwGSwCVLrv-ERoBIClZjrEsrETVpytj_31S0oQ4IEhUwXDlBss-H5I5c_xzUkyEA";
        let r = decrypt_download_token(t).unwrap();
        eprintln!("r = {:#?}", r);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PasetoResult {
    pub iss: String,
    /// Not Before
    pub nbf: String,
    pub user_id: String,
    /// Expiry
    pub exp: String,
}

/// Decrypts a download token and returns the ID from within the token.
pub(crate) fn decrypt_download_token(token: &str) -> Option<String> {
    let decrypted = match paseto::v2::decrypt_paseto(token, None, PASETO_KEY) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("e = {:#?}", e);
            return None;
        }
    };

    let result = match serde_json::from_str::<PasetoResult>(&decrypted) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("e = {:#?}", e);
            return None;
        }
    };

    let valid = match chrono::DateTime::parse_from_rfc3339(&result.exp) {
        Ok(time) => time > Utc::now(),
        Err(e) => {
            eprintln!("e = {:#?}", e);
            return None;
        }
    };

    if !valid {
        eprintln!("past expiry date");
        return None;
    }

    // It is valid
    Some(result.user_id)
}
