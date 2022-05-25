use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post, web, Error, HttpResponse,
};
use entity::user;
use idgenerator::{IdGeneratorOptions, IdInstance};
use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use regex::Regex;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::info;

use crate::{
    utils::{self, ise},
    FROM_ADDR, PUBLIC_ADDR, SMTP_ADDR, SMTP_PASSWORD, SMTP_USERNAME,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserEnrollmentPayload {
    pub email: String,
    pub name: Option<String>,
}

#[post("/enrol")]
pub(crate) async fn enrol_user(
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

    // Generate the password
    let hash_result = crate::utils::get_password_from_id(id);

    // Send the email
    let email = EmailBuilder::new()
        .to(data.email.clone())
        .from((FROM_ADDR.to_string(), "Security Challenges Platform"))
        .subject("COMP6443 Client Certificates").text(format!(r#"
Attached is your client certificate for COMP6443 at UNSW. You will have to download this pfx archive and import it into your keychain.

Your link to download the archive is here: {}

It is valid for 30 minutes. The password to install the pfx archive is {}. Do not share these certificates with anyone else, as they will be able to access your account.
        "#, link, hash_result)).build().map_err(ise!("EUBE"))?;

    let mut mailer = SmtpClient::new_simple(SMTP_ADDR.as_str())
        .map_err(ise!("EUCM"))?
        .credentials(Credentials::new(
            SMTP_USERNAME.as_str().to_owned(),
            SMTP_PASSWORD.as_str().to_owned(),
        ))
        .transport();

    let r = mailer.send(email.into()).map_err(ise!("EUSE"))?;
    info!("send email response = {:#?}", r);

    Ok(HttpResponse::Ok().finish())
}
