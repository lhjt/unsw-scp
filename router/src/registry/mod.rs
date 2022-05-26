use router_entity::entities::service;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use thiserror::Error;
use tracing::{error, warn};

use crate::gaia_utils;

pub mod services;

#[derive(Debug, Clone, Error)]
pub(crate) enum EvaluationErrors {
    #[error("The user does not have permission to access the service.")]
    Forbidden,
    #[error("There is no service that exists at this endpoint.")]
    NotFound,
    #[error("The URI that was supplied did not have a valid host.")]
    InvalidUriError,
    #[error("User did not have relevant roles.")]
    NoRoles,
    #[error("An internal error occurred.")]
    InternalError,
}

/// Determine which address a supplied URI should be proxied to.
#[tracing::instrument]
pub(crate) async fn evaluate_uri(
    uri: url::Url,
    token: &str,
    conn: &DatabaseConnection,
) -> Result<url::Url, EvaluationErrors> {
    if !uri.has_host() {
        return Err(EvaluationErrors::InvalidUriError);
    }

    let roles = gaia_utils::get_roles(token).await.map_err(|e| {
        warn!(
            "user has no roles or there was an error fetching them: {:?}",
            e
        );
        EvaluationErrors::NoRoles
    })?;

    // Attempt to find a service with the external hostname from the database
    let service = service::Entity::find()
        .filter(service::Column::ExternalHostname.eq(uri.host().unwrap().to_string()))
        .one(conn)
        .await
        .map_err(|e| {
            error!("failed to query database: {}", e);
            EvaluationErrors::InternalError
        })?
        .ok_or(EvaluationErrors::NotFound)?;

    // Create a new modified url that has the new host set appropriately and no path components
    let new_uri =
        url::Url::parse(&format!("http://{}", service.internal_hostname)).map_err(|e| {
            error!("failed to parse destination hostname: {}", e);
            EvaluationErrors::InternalError
        })?;

    let not_admin = !roles.contains("tutor") && !roles.contains("admin");

    // Determine if the user is allowed to access this service
    if let Some(dt) = service.not_before {
        if chrono::offset::Utc::now().le(&dt) {
            // Determine if the user has appropriate access rights
            if not_admin {
                // The user should not be able to access the service right now
                // Return not allowed
                return Err(EvaluationErrors::Forbidden);
            }

            // Otherwise they should be allowed to pass
            return Ok(new_uri);
        }
    }

    // Determine if we are passed the last date that submissions could be entered
    if let Some(dt) = service.not_after {
        if chrono::offset::Utc::now().ge(&dt) && not_admin {
            return Err(EvaluationErrors::Forbidden);
        }
    }

    Ok(new_uri)
}
