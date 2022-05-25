use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get,
    http::header::{self, ContentType, DispositionParam},
    post, web, Error, HttpResponse,
};
use entity::user;
use idgenerator::{IdGeneratorOptions, IdInstance};
use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    utils::{self, ise},
    CA_CERT, CA_KEY, FROM_ADDR, PUBLIC_ADDR, SMTP_ADDR, SMTP_PASSWORD, SMTP_USERNAME,
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
    let hash_result = crate::utils::get_password_from_id(&format!("_scpU{}@unsw.scp.platform", id));

    // Send the email
    let email = EmailBuilder::new()
        .to(data.email.clone())
        .from((FROM_ADDR.to_string(), "UNSW Security Challenges Platform"))
        .subject("COMP6443 Client Certificates").text(format!(r#"
Attached is your client certificate for COMP6443 at UNSW. You will have to download this certificate archive and import it into your keychain.

Your link to download your certificate package is: {}

- It is valid for 30 minutes.
- The password to install the pfx archive is: {}
- Do not share these certificates with anyone else, as they will be able to access your account.
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

#[derive(Debug, Deserialize)]
pub(crate) struct DownloadTokenQueryParams {
    pub key: String,
}

#[get("/download")]
pub(crate) async fn download_certs(
    conn: web::Data<DatabaseConnection>,
    query_params: web::Query<DownloadTokenQueryParams>,
) -> Result<HttpResponse, Error> {
    // Extract and validate token claims
    let claims = utils::tokens::decrypt_download_token(&query_params.key).ok_or_else(|| {
        ErrorBadRequest("Invalid download token. Please request your certificates again.")
    })?;

    // Check if the email has already been used
    let already_generated = user::Entity::find()
        .filter(user::Column::Email.eq(claims.signup_email.clone()))
        .one(conn.as_ref())
        .await
        .map_err(ise!("DCFO"))?
        .is_some();

    // Generate password
    let password = utils::get_password_from_id(&claims.user_id);

    // Generate certificates
    let cert =
        cert_utils::create_client_cert("COMP6443-unnamed".to_string(), claims.user_id.clone())
            .map_err(ise!("DCCCC"))?;

    // Sign certificates
    let ca_cert = cert_utils::get_ca_cert(&CA_CERT, &CA_KEY).map_err(ise!("DCGCC"))?;
    let client_pfx = cert_utils::generate_pfx(&cert, &ca_cert, "6443-certificates", &password)
        .map_err(ise!("DCGPX"))?;

    if !already_generated {
        let txn = conn.begin().await.map_err(ise!("DCBTX"))?;
        let uid = claims
            .user_id
            .strip_prefix("_scpU")
            .unwrap()
            .strip_suffix("@unsw.scp.platform")
            .unwrap()
            .to_string();

        // Update database
        let user = entity::user::ActiveModel {
            email: sea_orm::ActiveValue::Set(claims.signup_email),
            user_id: sea_orm::ActiveValue::Set(uid.clone()),
            ..Default::default()
        };

        user.insert(&txn).await.map_err(ise!("DCIUM"))?;
        // Add student role by default
        let role = entity::role::ActiveModel {
            name: sea_orm::ActiveValue::Set("student".to_string()),
            user_id: sea_orm::ActiveValue::Set(uid.clone()),
            ..Default::default()
        };

        role.insert(&txn).await.map_err(ise!("DCIRR"))?;

        // Commit transaction
        txn.commit().await.map_err(ise!("DCCTX"))?;
    }

    // Send cert to client
    Ok(HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .insert_header(header::ContentDisposition {
            disposition: header::DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename("certificates.pfx".to_string())],
        })
        .body(client_pfx))
}
