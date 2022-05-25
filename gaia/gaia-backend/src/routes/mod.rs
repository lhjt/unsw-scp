use std::collections::HashSet;

use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError},
    get, post, web, Error, HttpRequest, HttpResponse,
};
use sea_orm::DatabaseConnection;

use crate::utils::{self, get_token_id, ise};

pub mod self_service;

#[get("/user/{id}/roles")]
pub(crate) async fn get_user_roles(
    req: HttpRequest,
    user_id: web::Path<String>,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // First ensure that the user making this request has the "admin" role
    if !utils::get_roles(&get_token_id(&req)?, &conn)
        .await
        .map_err(ise!("GR"))?
        .contains("admin")
    {
        return Err(ErrorForbidden(
            "You do not have permission to perform this action.",
        ));
    }

    let roles: HashSet<String> = utils::get_roles(&user_id, conn.into_inner().as_ref())
        .await
        .map_err(ise!("GR"))?;

    Ok(HttpResponse::Ok().json(roles))
}

#[post("/user/{id}/roles")]
pub(crate) async fn set_user_roles(
    req: HttpRequest,
    user_id: web::Path<String>,
    new_roles: web::Json<Vec<String>>,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // First ensure that the user making this request has the "admin" role
    if !utils::get_roles(&get_token_id(&req)?, &conn)
        .await
        .map_err(ise!("GR"))?
        .contains("admin")
    {
        return Err(ErrorForbidden(
            "You do not have permission to perform this action.",
        ));
    }

    // Make changes
    utils::set_roles(&user_id, new_roles.0, &conn).await?;

    Ok(HttpResponse::Ok().finish())
}
