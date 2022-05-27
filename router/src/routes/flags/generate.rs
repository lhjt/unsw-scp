use actix_web::{
    error::{ErrorBadGateway, ErrorNotFound},
    get,
    web,
    Error,
    HttpRequest,
    HttpResponse,
};
use hmac::{Hmac, Mac};
use router_entity::flag::{self, FlagType};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Deserialize;
use sha2::Sha256;

use crate::{handler_utils::ise, HMAC_KEY};

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
    let claims = crate::handler_utils::get_claims(&req)?;
    // Get the user id/email
    let email = claims.user_id;

    // Get the flag from the database
    let found_flag = flag::Entity::find_by_id(params.id.clone())
        .one(conn.as_ref())
        .await
        .map_err(ise!("GFFFI"))?
        .ok_or_else(|| ErrorNotFound("Supplied flag ID does not exist"))?;

    if let FlagType::Static = found_flag.flag_type {
        return Err(ErrorBadGateway("Cannot generate a flag for a static flag"));
    }

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
