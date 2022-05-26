use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EvaluateRequestPayload {
    /// The uri of the request that needs to be routed.
    pub(crate) uri: String,
}

#[post("/evaluate")]
pub(crate) async fn evaluate(
    req: HttpRequest,
    payload: web::Json<EvaluateRequestPayload>,
) -> Result<HttpResponse, Error> {
    // Get roles for this user
    let roles: Vec<String> = todo!();

    // Search the registry to determine where the request should go
    let destination: Option<url::Url> = todo!();

    // Return based on routing
    // If not allowed then send back 403

    todo!()
}
