use std::collections::HashSet;

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    Error, HttpRequest,
};
use entity::role;
use intra_jwt::ClaimsData;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::JWT_PEM;

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
