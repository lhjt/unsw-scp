/// Macro to quickly construct an internal server error with an error code.
macro_rules! ise {
    ($code:expr) => {
        |e| {
            tracing::error!("exception occurred: {}", e);
            actix_web::error::ErrorInternalServerError(concat!("Internal server error: EC.", $code))
        }
    };
}

use actix_web::{
    error::{ErrorForbidden, ErrorUnauthorized},
    Error,
    HttpRequest,
};
use intra_jwt::ClaimsData;
pub(crate) use ise;

use crate::JWT_PEM;

/// Get claims from a HTTP request's auth token.
pub(crate) fn get_claims(req: &HttpRequest) -> Result<ClaimsData, Error> {
    // Get the auth token
    let token = req
        .headers()
        .get("X-Scp-Auth")
        .ok_or_else(|| ErrorUnauthorized("Missing authentication"))?
        .to_str()
        .map_err(ise!("GCEAH"))?;

    // Process the token into claims
    intra_jwt::verify_jwt(token, JWT_PEM.as_str())
        .map_err(|_| ErrorForbidden("Unable to get claims from auth token"))
}
