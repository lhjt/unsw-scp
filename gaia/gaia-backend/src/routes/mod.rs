use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    get, web, Error, HttpRequest, HttpResponse,
};
use entity::role;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[get("/roles")]
pub(crate) async fn get_roles(
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // Get JWT
    let jwt_str = req
        .headers()
        .get("x-scp-auth")
        .ok_or_else(|| ErrorUnauthorized("Missing authentication token"))?
        .to_str()
        .map_err(ErrorInternalServerError)?;

    // Validate str
    let claims = intra_jwt::verify_jwt(
        jwt_str,
        &std::fs::read_to_string("../../proxy/certs/jwt-key.pem").unwrap(),
    )
    .map_err(|_| ErrorBadRequest("Invalid authentication token"))?;

    let email = claims.user_id;

    // Get user id from email
    let id = email
        .split('+')
        .next()
        .ok_or_else(|| ErrorBadRequest("Invalid authentication token"))?
        .strip_prefix("_scpU")
        .ok_or_else(|| ErrorBadRequest("Invalid authentication token"))?;

    // Get roles for id
    let roles: Vec<String> = role::Entity::find()
        .filter(role::Column::UserId.eq(id))
        .all(conn.into_inner().as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .iter()
        .map(|r| r.name.to_owned())
        .collect();

    Ok(HttpResponse::Ok().json(roles))
}
