use std::collections::HashSet;

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    Error, HttpRequest,
};
use entity::{role, user};
use intra_jwt::ClaimsData;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};

use crate::JWT_PEM;
pub mod tokens;

/// Macro to quickly construct an internal server error with an error code.
macro_rules! ise {
    ($code:expr) => {
        |_| ErrorInternalServerError(concat!("Internal server error: EC.", $code))
    };
}

pub(crate) use ise;

/// Fetch the auth claims from a http request's token.
pub(crate) fn get_auth_claims(req: &HttpRequest) -> Result<ClaimsData, Error> {
    // Get JWT
    let jwt_str = req
        .headers()
        .get("x-scp-auth")
        .ok_or_else(|| ErrorUnauthorized("Missing authentication token"))?
        .to_str()
        .map_err(ErrorInternalServerError)?;

    // Validate str
    intra_jwt::verify_jwt(jwt_str, &JWT_PEM)
        .map_err(|_| ErrorBadRequest("Invalid authentication token"))
}

/// Gets the user's id from the certificate.
pub(crate) fn get_token_id(req: &HttpRequest) -> Result<String, Error> {
    let claims = get_auth_claims(req)?;

    let email = claims.user_id;

    // Get user id from email
    let id = email
        .split('+')
        .next()
        .ok_or_else(|| ErrorBadRequest("Invalid authentication token"))?
        .strip_prefix("_scpU")
        .ok_or_else(|| ErrorBadRequest("Invalid authentication token"))?
        .to_owned();

    Ok(id)
}

/// Get roles for a user id.
pub(crate) async fn get_roles(
    id: &str,
    conn: &DatabaseConnection,
) -> anyhow::Result<HashSet<String>> {
    Ok(role::Entity::find()
        .filter(role::Column::UserId.eq(id))
        .all(conn)
        .await?
        .iter()
        .map(|r| r.name.to_owned())
        .collect())
}

/// Set the roles for a user.
pub(crate) async fn set_roles(
    id: &str,
    roles: Vec<String>,
    conn: &DatabaseConnection,
) -> anyhow::Result<bool, Error> {
    user::Entity::find()
        .filter(user::Column::UserId.eq(id))
        .one(conn)
        .await
        .map_err(ise!("SRDBQ"))?
        .ok_or_else(|| ErrorNotFound(format!("The user with id {} does not exist", id)))?;

    let txn = conn.begin().await.map_err(ise!("SRBTXN"))?;

    // Remove all previous roles
    role::Entity::delete_many()
        .filter(role::Column::UserId.eq(id))
        .exec(&txn)
        .await
        .map_err(ise!("SRDM"))?;

    // Insert new roles for this user
    role::Entity::insert_many(roles.into_iter().map(|r| role::ActiveModel {
        name: Set(r),
        user_id: Set((*id).to_string()),
        ..Default::default()
    }))
    .exec(&txn)
    .await
    .map_err(ise!("SRIM"))?;

    // Commit transaction
    txn.commit().await.map_err(ise!("SRCTX"))?;

    Ok(true)
}
