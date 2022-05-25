use crate::utils::{self, get_token_id, ise};
use actix_web::{error::ErrorInternalServerError, get, web, Error, HttpRequest, HttpResponse};
use sea_orm::DatabaseConnection;
use std::collections::HashSet;

#[get("/selfserve/id")]
pub(crate) async fn get_id(req: HttpRequest) -> Result<HttpResponse, Error> {
    let id = get_token_id(&req)?;

    Ok(HttpResponse::Ok().body(id))
}

#[get("/selfserve/roles")]
pub(crate) async fn get_roles(
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // Get user id from email
    let id = get_token_id(&req)?;

    // Get roles for id
    let roles: HashSet<String> = utils::get_roles(&id, conn.into_inner().as_ref())
        .await
        .map_err(ise!("GR"))?;

    Ok(HttpResponse::Ok().json(roles))
}
