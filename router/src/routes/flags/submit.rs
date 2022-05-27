use actix_web::{
    error::{ErrorBadRequest, ErrorNotFound},
    post,
    web,
    Error,
    HttpRequest,
    HttpResponse,
};
use hmac::{Hmac, Mac};
use idgenerator::{IdGeneratorOptions, IdInstance};
use regex::Regex;
use router_entity::{
    flag::{self, FlagType},
    submission,
    user,
};
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    Set,
    TransactionTrait,
};
use serde::Deserialize;
use sha2::Sha256;

use crate::{
    handler_utils::{self, ise},
    HMAC_KEY,
};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SubmitFlagPayload {
    pub(crate) flag: String,
}

#[post("/{id}/submit")]
pub(crate) async fn submit_flag(
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
    flag_id: web::Path<String>,
    flag_payload: web::Json<SubmitFlagPayload>,
) -> Result<HttpResponse, Error> {
    // Get the auth token
    let claims = handler_utils::get_claims(&req)?;
    // Get the user id/email
    let email = claims.user_id;

    // Ensure that the supplied flag matches valid regex
    let regex = Regex::new(r"(?m)COMP6443\{(((.+?)\.(.+?)\.(.+?))|(.+?))\}").unwrap();
    if !regex.is_match(flag_payload.flag.as_str()) {
        return Err(ErrorBadRequest("1 Invalid flag provided"));
    }

    // Check if this flag name correlates with this flag id
    let actual_flag = flag::Entity::find_by_id(flag_id.clone())
        .one(conn.as_ref())
        .await
        .map_err(ise!("SFFFV"))?
        .ok_or_else(|| ErrorNotFound("Flag does not exist"))?;

    match actual_flag.flag_type {
        FlagType::Static => {
            let flag_name = flag_payload
                .flag
                .strip_prefix("COMP6443{")
                .unwrap()
                .strip_suffix('}')
                .unwrap();

            if actual_flag.flag != flag_name {
                return Err(ErrorBadRequest("2 Invalid flag provided"));
            }
        },
        FlagType::Dynamic => {
            let regex = Regex::new(r"(?m)COMP6443\{(.+?)\.(.+?)\.(.+?)\}").unwrap();
            if !regex.is_match(flag_payload.flag.as_str()) {
                return Err(ErrorBadRequest("3 Invalid flag provided"));
            }

            let mut components = flag_payload
                .flag
                .strip_prefix("COMP6443{")
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split('.');
            let flag_name = components.next().unwrap();
            if actual_flag.flag != flag_name {
                return Err(ErrorBadRequest("4 Invalid flag provided"));
            }

            // Check middle component
            let middle = base64::decode(components.next().unwrap())
                .map_err(|_| ErrorBadRequest("5 Invalid flag provided"))?;
            let middle = std::str::from_utf8(&middle)
                .map_err(|_| ErrorBadRequest("6 Invalid flag provided"))?;
            if middle != email {
                return Err(ErrorBadRequest("7 Invalid flag provided"));
            }

            // Validate hmac
            let final_component = components.next().unwrap();
            // Hash the username and flag id together
            let mut mac = HmacSha256::new_from_slice(HMAC_KEY.as_bytes()).unwrap();
            mac.update(format!("{}_{}", email, flag_id).as_bytes());
            let result = mac.finalize();
            let signature = base64::encode(result.into_bytes());

            if signature != final_component {
                return Err(ErrorBadRequest("8 Invalid flag provided"));
            }
        },
    }

    let uid = email
        .strip_prefix("_scpU")
        .unwrap()
        .strip_suffix("@unsw.scp.platform")
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap();

    // Determine if the user has already submitted a flag for this
    // So far so good; store the submission in the database
    let txn = conn.begin().await.map_err(ise!("SFSTX"))?;
    if user::Entity::find_by_id(uid)
        .one(&txn)
        .await
        .map_err(ise!("SFGU"))?
        .is_none()
    {
        let new_user = user::ActiveModel { id: Set(uid) };
        new_user.insert(&txn).await.map_err(ise!("SFCNU"))?;
    }

    // Determine if the user has already submitted this flag
    if submission::Entity::find()
        .filter(submission::Column::FlagId.eq(actual_flag.id.clone()))
        .filter(submission::Column::UserId.eq(uid))
        .one(&txn)
        .await
        .map_err(ise!("SFFES"))?
        .is_some()
    {
        return Err(ErrorBadRequest("The user has already submitted this flag"));
    };

    // Setup id generator
    let generator_options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    IdInstance::init(generator_options).map_err(ise!("CIG"))?;

    // Create a new submission
    let new_submission = submission::ActiveModel {
        flag_id:         Set(actual_flag.id),
        id:              Set(IdInstance::next_id()),
        submission_time: Set(chrono::offset::Utc::now()),
        user_id:         Set(uid),
    };
    new_submission.insert(&txn).await.map_err(ise!("SFCNS"))?;

    // Commit
    txn.commit().await.map_err(ise!("SFCTX"))?;
    Ok(HttpResponse::Accepted().finish())
}
