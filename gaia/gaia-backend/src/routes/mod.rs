use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    get, Error, HttpRequest, HttpResponse,
};

#[get("/roles")]
pub(crate) async fn get_roles(req: HttpRequest) -> Result<HttpResponse, Error> {
    // Get JWT
    let jwt_str = req
        .headers()
        .get("x-scp-auth")
        .ok_or_else(|| ErrorUnauthorized("Missing authentication token"))?
        .to_str()
        .map_err(ErrorInternalServerError)?;

    // Validate str
    let claims = intra_jwt::verify_jwt(jwt_str, todo!())
        .map_err(|_| ErrorBadRequest("Invalid authentication token"))?;

    let email = claims.user_id;

    // Get user id from email
    let id = email
        .split('+')
        .next()
        .ok_or_else(|| ErrorInternalServerError("Internal server error: EC.SBP"))?
        .strip_prefix("_scpU")
        .ok_or_else(|| ErrorBadRequest("Invalid authentication token"))?;

    // Get roles for id
    let roles: Vec<String> = todo!();

    Ok(HttpResponse::Ok().json(roles))
}
