use std::collections::{HashMap, HashSet};

use actix_web::{
    error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError, ErrorUnauthorized},
    post, web, Error, HttpRequest, HttpResponse,
};
use chrono::Utc;
use idgenerator::{IdGeneratorOptions, IdInstance};
use router_entity::{category, challenge, flag, service};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::registry::services::{validate_flags, validate_services};

/// Macro to quickly construct an internal server error with an error code.
macro_rules! ise {
    ($code:expr) => {
        |e| {
            tracing::error!("exception occurred: {}", e);
            ErrorInternalServerError(concat!("Internal server error: EC.", $code))
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NewService {
    pub(crate) name: String,
    /// The category that the service is part of.
    pub(crate) category: String,
    /// The date before which students cannot access the challenge.
    pub(crate) nbf: Option<chrono::DateTime<Utc>>,
    /// The date after which students cannot access the challenge.
    pub(crate) naf: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NewFlag {
    /// The flag's type. Should be either `static` or `dynamic`.
    pub(crate) flag_type: String,
    /// The unique ID of the flag. Should be unique across all flags.
    pub(crate) id: String,
    pub(crate) display_name: String,
    pub(crate) category: String,
    pub(crate) points: i32,
    /// The flag that will be used as part of the flag generation process.
    /// If the flag type is `static`, it will be the actual flag that is submitted.
    pub(crate) flag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NewServicePayload {
    pub(crate) services: Vec<NewService>,
    pub(crate) flags: Vec<NewFlag>,
}

#[tracing::instrument]
#[post("/services")]
pub(crate) async fn create_service(
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
    payload: web::Json<NewServicePayload>,
) -> Result<HttpResponse, Error> {
    let token = req
        .headers()
        .get("x-scp-auth")
        .ok_or_else(|| ErrorUnauthorized(""))?
        .to_str()
        .map_err(ErrorForbidden)?;

    // Get the roles for the user
    let roles = crate::gaia_utils::get_roles(token)
        .await
        .map_err(ErrorInternalServerError)?;

    // Determine if the user has enough permissions to create new services
    if !roles.contains("admin") {
        return Err(ErrorForbidden(""));
    }

    // Validate the service entries
    if !validate_services(&payload.services) {
        return Err(ErrorBadRequest("Invalid service definitions"));
    }

    // Validate the flag entries
    if !validate_flags(&payload.flags) {
        return Err(ErrorBadRequest("Invalid flag definitions"));
    }

    // Create a new transaction
    let txn = conn.begin().await.map_err(ErrorInternalServerError)?;

    // Setup id generator
    let generator_options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    IdInstance::init(generator_options).map_err(ise!("CIG"))?;
    let new_challenge_id = IdInstance::next_id();

    // Create a new challenge
    let new_challenge = challenge::ActiveModel {
        id: Set(new_challenge_id),
    };
    new_challenge.insert(&txn).await.map_err(ise!("CSCNC"))?;

    // Create new categories if they do not exist
    let categories_to_match = payload
        .services
        .iter()
        .map(|s| s.category.as_str())
        .chain(payload.flags.iter().map(|f| f.category.as_str()))
        .collect::<HashSet<&str>>();

    let existing: HashMap<String, category::Model> = category::Entity::find()
        .filter(category::Column::Name.is_in(categories_to_match.clone()))
        .all(&txn)
        .await
        .map_err(ise!("CSGEC"))?
        .into_iter()
        .map(|m| (m.name.clone(), m))
        .collect();

    let mut category_name_id_map: HashMap<String, i64> = HashMap::new();

    let new_categories: Vec<category::ActiveModel> = categories_to_match
        .iter()
        .filter_map(|c| {
            if existing.contains_key(*c) {
                category_name_id_map.insert((*c).to_string(), existing.get(*c).unwrap().id);
                None
            } else {
                let new_id = IdInstance::next_id();
                category_name_id_map.insert((*c).to_string(), new_id);
                Some(category::ActiveModel {
                    id: Set(new_id),
                    name: Set((*c).to_string()),
                })
            }
        })
        .collect();

    if !new_categories.is_empty() {
        // Insert these newly generates categories into the db
        category::Entity::insert_many(new_categories)
            .exec(&txn)
            .await
            .map_err(ise!("CSINC"))?;
    }

    // Ensure that all internal hostnames are unique
    let internal_names: HashSet<String> = payload
        .services
        .iter()
        .map(|s| format!("{}.challenges.svc.cluster.local", s.name))
        .collect();
    if !service::Entity::find()
        .filter(service::Column::InternalHostname.is_in(internal_names.clone()))
        .all(&txn)
        .await
        .map_err(ise!("CSQSI"))?
        .is_empty()
    {
        return Err(ErrorBadRequest(format!(
            "Cannot create a service with a name that already exists: {:?}",
            internal_names
        )));
    }

    // Insert new services into the database
    let new_services = payload
        .services
        .iter()
        .map(|s| service::ActiveModel {
            id: Set(IdInstance::next_id()),
            category_id: Set(*category_name_id_map.get(&s.category).unwrap()),
            challenge_id: Set(new_challenge_id),
            external_hostname: Set(s.name.clone()),
            internal_hostname: Set(format!("{}.challenges.svc.cluster.local", s.name)),
            name: Set(s.name.clone()),
            not_after: Set(s.naf),
            not_before: Set(s.nbf),
        })
        .collect::<Vec<service::ActiveModel>>();

    service::Entity::insert_many(new_services)
        .exec(&txn)
        .await
        .map_err(ise!("CSINS"))?;

    if !payload.flags.is_empty() {
        // Ensure that there are no records in the database that have the same flag id
        let new_flag_ids: HashSet<String> = payload.flags.iter().map(|f| f.id.clone()).collect();
        let duplicate_flags = flag::Entity::find()
            .filter(flag::Column::Id.is_in(new_flag_ids))
            .all(&txn)
            .await
            .map_err(ise!("CSQAF"))?;
        if !duplicate_flags.is_empty() {
            return Err(ErrorBadRequest(format!(
                "Attempted to insert duplicate flags into the database: {:?}",
                duplicate_flags
                    .into_iter()
                    .map(|f| f.id)
                    .collect::<Vec<String>>()
            )));
        }

        // Insert new flags into the database
        let new_flags = payload
            .flags
            .iter()
            .map(|f| flag::ActiveModel {
                category_id: Set(*category_name_id_map.get(&f.category).unwrap()),
                challenge_id: Set(new_challenge_id),
                flag: Set(f.flag.clone()),
                flag_type: Set(match f.flag_type.as_str() {
                    "static" => flag::FlagType::Static,
                    "dynamic" => flag::FlagType::Dynamic,
                    v => unreachable!("got: {}", v),
                }),
                id: Set(f.id.clone()),
                points: Set(f.points),
                display_name: Set(f.display_name.clone()),
            })
            .collect::<Vec<flag::ActiveModel>>();

        flag::Entity::insert_many(new_flags)
            .exec(&txn)
            .await
            .map_err(ise!("CSINF"))?;
    }

    // Commit transaction
    txn.commit().await.map_err(ise!("CSCFT"))?;

    Ok(HttpResponse::Ok().finish())
}
