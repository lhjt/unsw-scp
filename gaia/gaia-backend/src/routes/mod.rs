use std::collections::HashSet;

use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError},
    get, post, web, Error, HttpRequest, HttpResponse,
};
use sea_orm::DatabaseConnection;

use crate::utils::{self, get_token_id, ise};

#[get("/selfserve/id")]
pub(crate) async fn ss_get_id(req: HttpRequest) -> Result<HttpResponse, Error> {
    let id = get_token_id(&req)?;

    Ok(HttpResponse::Ok().body(id))
}

#[get("/selfserve/roles")]
pub(crate) async fn ss_get_roles(
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
