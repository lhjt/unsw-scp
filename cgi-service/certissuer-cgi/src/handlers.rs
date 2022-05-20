use std::ops::Deref;

use http::HeaderValue;
use http::StatusCode;

use crate::errors;
use crate::tokens;

macro_rules! ise {
    ($code:expr) => {
        cgi::text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            concat!(stringify!($code), ": Internal server error"),
        )
    };
}

macro_rules! bad_req {
    ($reason:expr) => {
        cgi::string_response(StatusCode::BAD_REQUEST, $reason)
    };
}

pub(crate) fn generate_certificate_handler(query: &str) -> http::Response<Vec<u8>> {
    let token = match querystring::querify(query)
        .iter()
        .find(|(key, _)| key == &"dt")
    {
        Some((_, v)) => v.deref().to_string(),
        None => return bad_req!("Invalid token"),
    };
    let result = match tokens::decrypt_download_token(&token) {
        Some(id) => id,
        None => return bad_req!("Invalid request"),
    };
    let ca_cert_loc =
        std::env::var("CA_CERT_LOCATION").unwrap_or_else(|_| "../rootCA.pem".to_string());
    let ca_key_loc =
        std::env::var("CA_KEY_LOCATION").unwrap_or_else(|_| "../rootCA-key.pem".to_string());
    let ca_cert_pem = match std::fs::read_to_string(ca_cert_loc) {
        Ok(pem) => pem,
        Err(e) => {
            errors::handle_error(e);
            return ise!("EC.CCPR");
        }
    };
    let ca_key_pem = match std::fs::read_to_string(ca_key_loc) {
        Ok(pem) => pem,
        Err(e) => {
            errors::handle_error(e);
            return ise!("EC.CKPR");
        }
    };
    let ca_cert = match cert_utils::get_ca_cert(&ca_cert_pem, &ca_key_pem) {
        Ok(c) => c,
        Err(_e) => {
            // handle_error(e);
            return ise!("EC.GCC");
        }
    };
    let new_cert = match cert_utils::create_client_cert("username".to_string(), result) {
        Ok(c) => c,
        Err(_e) => return ise!("EC.CCC"),
    };
    let pfx = match cert_utils::generate_pfx(&new_cert, &ca_cert, "COMP6443", "6443-certpkg") {
        Ok(pfx) => pfx,
        Err(_e) => {
            // handle_error(_e)
            return ise!("EC.PFXG");
        }
    };
    http::Response::builder()
        .header(
            "Content-Type",
            HeaderValue::from_str("application/octet-stream").unwrap(),
        )
        .header(
            "Content-Disposition",
            HeaderValue::from_str("attachment; filename=\"6443-certificate.pfx\"").unwrap(),
        )
        .body(pfx)
        .unwrap()
}
