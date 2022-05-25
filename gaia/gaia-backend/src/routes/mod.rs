use std::collections::HashSet;

use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError},
    get, post, web, Error, HttpRequest, HttpResponse,
};
use entity::{role, user};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserWithRole {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub roles: Vec<String>,
}

#[get("/users")]
pub(crate) async fn get_users(
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // First ensure that the user making this request has the "tutor" or "admin" role
    let roles = utils::get_roles(&get_token_id(&req)?, &conn)
        .await
        .map_err(ise!("GR"))?;
    if !roles.contains("admin") && !roles.contains("tutor") {
        return Err(ErrorForbidden(
            "You do not have permission to perform this action.",
        ));
    }

    // Get all users and their roles
    let users_with_roles: Vec<UserWithRole> = user::Entity::find()
        .find_with_related(role::Entity)
        .all(conn.as_ref())
        .await
        .map_err(ise!("AUFWR"))?
        .into_iter()
        .map(|(user, roles)| UserWithRole {
            id: user.user_id,
            email: user.email,
            name: user.name,
            roles: roles.into_iter().map(|f| f.name).collect(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(users_with_roles))
}
