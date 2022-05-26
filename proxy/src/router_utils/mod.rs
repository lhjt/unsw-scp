use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

use crate::ROUTER_URL;

#[derive(Debug, Clone, Error)]
pub(crate) enum EvaluationErrors {
    #[error("The user does not have permission to access the service.")]
    Forbidden,
    #[error("There is no service that exists at this endpoint.")]
    NotFound,
    #[error("The URI that was supplied did not have a valid host.")]
    InvalidUriError,
    #[error("An internal error occurred.")]
    InternalError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvaluateResponsePayload {
    pub(crate) new_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvaluateRequestPayload {
    pub(crate) uri: String,
}

/// Get the uri to where a request should be proxied to by ctf subdomain.
#[tracing::instrument]
pub(crate) async fn get_route(subdomain: &str, token: &str) -> Result<url::Url, EvaluationErrors> {
    let request_uri = url::Url::parse(&format!("http://{}", subdomain)).map_err(|e| {
        error!("failed to parse url: {:?}", e);
        EvaluationErrors::InternalError
    })?;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/api/evaluate", ROUTER_URL.as_str()))
        .header("X-Scp-Auth", token)
        .json(&EvaluateRequestPayload {
            uri: request_uri.to_string(),
        })
        .send()
        .await
        .map_err(|e| {
            error!("failed to make request: {:?}", e);
            EvaluationErrors::InternalError
        })?;

    match res.status() {
        StatusCode::OK => url::Url::parse(
            &res.json::<EvaluateResponsePayload>()
                .await
                .map_err(|e| {
                    error!("failed to deserialise response: {:?}", e);
                    EvaluationErrors::InternalError
                })?
                .new_uri,
        )
        .map_err(|e| {
            error!("failed to parse response uri: {:?}", e);
            EvaluationErrors::InternalError
        }),
        StatusCode::FORBIDDEN => Err(EvaluationErrors::Forbidden),
        StatusCode::NOT_FOUND => Err(EvaluationErrors::NotFound),
        StatusCode::BAD_REQUEST => Err(EvaluationErrors::InvalidUriError),
        _ => {
            error!("received an error status code: {:?}", res.status());
            Err(EvaluationErrors::InternalError)
        },
    }
}
