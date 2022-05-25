use std::collections::HashSet;

use actix_web::{error::ErrorInternalServerError, get, web, Error, HttpRequest, HttpResponse};
use sea_orm::DatabaseConnection;

use crate::utils::get_token_id;

#[get("/roles")]
pub(crate) async fn get_roles(
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // Get user id from email
    let id = get_token_id(&req)?;

    // Get roles for id
    let roles: HashSet<String> = crate::utils::get_roles(&id, conn.into_inner().as_ref())
        .await
        .map_err(|_| ErrorInternalServerError("Internal server error: EC.GR"))?;

    Ok(HttpResponse::Ok().json(roles))
}
