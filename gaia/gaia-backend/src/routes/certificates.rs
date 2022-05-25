use std::time::{Duration, UNIX_EPOCH};

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post, web, Error, HttpRequest, HttpResponse,
};
use entity::user;
use idgenerator::{IdGeneratorOptions, IdInstance};
use regex::Regex;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    utils::{self, ise},
    PUBLIC_ADDR,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserEnrollmentPayload {
    pub email: String,
    pub name: Option<String>,
}

#[post("/enrol")]
pub(crate) async fn enrol_user(
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
    data: web::Json<UserEnrollmentPayload>,
) -> Result<HttpResponse, Error> {
    // If the user already exists in the database, they should not be allowed to resend themselves an email.
    // Users are added to the database once they download their file.
    if user::Entity::find()
        .filter(user::Column::Email.eq(data.0.email.clone()))
        .one(conn.as_ref())
        .await
        .map_err(ise!("EUFO"))?
        .is_some()
    {
        // User already exists
        // Send bad request
        return Err(ErrorBadRequest("This user has already downloaded their certificates. If this is an error, please contact the administrators."));
    }

    // User has not downloaded the certificates yet. Generate a download link and send them an email.
    // Validate that the email is either a UNSW email or CBA email.
    let regex = Regex::new(r"(?m)(((z\d{7})@unsw\.edu\.au)|((.+)@cba\.com\.au))").unwrap();
    if !regex.is_match(&data.email) {
        // Invalid email address; refuse
        return Err(ErrorBadRequest(
            "An email address that was not authorised by the system was received.",
        ));
    }

    // Generate the download link
    let generator_options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    let _ = IdInstance::init(generator_options).map_err(ise!("CIG"))?;
    let id = IdInstance::next_id();
    let token = utils::tokens::create_download_token(
        &format!("_scpU{}@unsw.scp.platform", id),
        &data.email,
    );
    let link = format!(
        "https://{}/api/certificates/download?key={}",
        PUBLIC_ADDR.as_str(),
        token
    );

    // Send the email

    todo!()
}
