use actix_web::{error::ErrorInternalServerError, get, Error, HttpRequest, HttpResponse};

#[get("/roles")]
pub(crate) async fn get_roles(req: HttpRequest) -> Result<HttpResponse, Error> {
    // Get JWT
    let jwt_str = match req.headers().get("x-scp-auth") {
        Some(value) => value.to_str().map_err(ErrorInternalServerError)?,
        None => return Ok(HttpResponse::Unauthorized().body("Missing authentication token")),
    };

    // Validate str
    let claims = match intra_jwt::verify_jwt(jwt_str, todo!()) {
        Ok(claims) => claims,
        Err(e) => return Ok(HttpResponse::BadRequest().body("Invalid authentication token")),
    };

    let email = claims.user_id;

    // Get user id from email
    let id = match email
        .split('+')
        .next()
        .ok_or_else(|| ErrorInternalServerError("Internal server error: EC.SBP"))?
        .strip_prefix("_scpU")
    {
        Some(id) => id,
        None => return Ok(HttpResponse::BadRequest().body("Invalid authentication token")),
    };

    // Get roles for id
    let roles: Vec<String> = todo!();

    Ok(HttpResponse::Ok().json(roles))
}
