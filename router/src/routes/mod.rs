use actix_web::{
    error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError, ErrorNotFound},
    post, web, Error, HttpRequest, HttpResponse,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::registry::{self, EvaluationErrors};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EvaluateRequestPayload {
    /// The uri of the request that needs to be routed.
    pub(crate) uri: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct EvaluationRequestResponse {
    pub(crate) new_uri: String,
}

impl EvaluationRequestResponse {
    pub(crate) fn new(new_uri: String) -> Self {
        Self { new_uri }
    }
}

#[post("/evaluate")]
pub(crate) async fn evaluate(
    req: HttpRequest,
    payload: web::Json<EvaluateRequestPayload>,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // Assumed safe with guards implemented
    let id = req
        .headers()
        .get("x-scp-auth")
        .unwrap()
        .to_str()
        .map_err(ErrorForbidden)?;

    let search_uri = url::Url::parse(&payload.uri).map_err(ErrorBadRequest)?;

    // Search the registry to determine where the request should go
    registry::evaluate_uri(search_uri, id, conn.as_ref())
        .await
        .map(|destination| {
            HttpResponse::Ok().json(EvaluationRequestResponse::new(destination.to_string()))
        })
        .map_err(|e| match e {
            EvaluationErrors::Forbidden => ErrorForbidden(""),
            EvaluationErrors::NotFound => ErrorNotFound(""),
            EvaluationErrors::InvalidUriError => ErrorBadRequest(""),
            _ => ErrorInternalServerError(""),
        })
}
