use actix_web::{
    error::{ErrorForbidden, ErrorNotFound, ErrorUnauthorized},
    get, web, Error, HttpRequest, HttpResponse,
};

use hmac::{Hmac, Mac};
use router_entity::flag;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Deserialize;
use sha2::Sha256;

use crate::{HMAC_KEY, JWT_PEM};

/// Macro to quickly construct an internal server error with an error code.
macro_rules! ise {
    ($code:expr) => {
        |e| {
            tracing::error!("exception occurred: {}", e);
            actix_web::error::ErrorInternalServerError(concat!("Internal server error: EC.", $code))
        }
    };
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GenerateFlagQueryParams {
    /// The id of the flag to generate.
    pub(crate) id: String,
}

type HmacSha256 = Hmac<Sha256>;

#[get("/generate")]
pub(crate) async fn generate_flag(
    req: HttpRequest,
    params: web::Query<GenerateFlagQueryParams>,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // Get the auth token
    let token = req
        .headers()
        .get("X-Scp-Auth")
        .ok_or_else(|| ErrorUnauthorized("Missing authentication"))?
        .to_str()
        .map_err(ise!("GFEAH"))?;

    // Process the token into claims
    let claims = intra_jwt::verify_jwt(token, JWT_PEM.as_str())
        .map_err(|_| ErrorForbidden("Unable to get claims from auth token"))?;

    // Get the user id/email
    let email = claims.user_id;

    // Get the flag from the database
    let found_flag = flag::Entity::find_by_id(params.id.clone())
        .one(conn.as_ref())
        .await
        .map_err(ise!("GFFFI"))?
        .ok_or_else(|| ErrorNotFound("Supplied flag ID does not exist"))?;

    // Hash the username and flag id together
    let mut mac = HmacSha256::new_from_slice(HMAC_KEY.as_bytes()).unwrap();
    mac.update(format!("{}_{}", email, params.id).as_bytes());
    let result = mac.finalize();
    let signature = base64::encode(result.into_bytes());

    let generated_flag = format!(
        "COMP6443{{{}.{}.{}}}",
        found_flag.flag,
        base64::encode(&email),
        signature
    );

    Ok(HttpResponse::Ok().body(generated_flag))
}
