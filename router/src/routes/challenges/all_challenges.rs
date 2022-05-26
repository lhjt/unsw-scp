use std::collections::HashMap;

use actix_web::{error::ErrorUnauthorized, get, web, Error, HttpRequest, HttpResponse};
use chrono::Utc;
use router_entity::{
    category, challenge,
    flag::{self, FlagType},
    service,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::handler_utils::ise;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReturnPayload {
    /// The ID of the challenge
    pub(crate) id: i64,
    pub(crate) services: Vec<ReturnService>,
    pub(crate) flags: Vec<ReturnFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReturnFlag {
    pub(crate) id: String,
    pub(crate) flag_type: FlagType,
    pub(crate) display_name: String,
    pub(crate) category: String,
    pub(crate) points: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReturnService {
    pub(crate) id: i64,
    pub(crate) category: String,
    pub(crate) name: String,
    pub(crate) not_before: Option<chrono::DateTime<Utc>>,
    pub(crate) not_after: Option<chrono::DateTime<Utc>>,
}

#[get("")]
pub(crate) async fn get_all(
    req: HttpRequest,
    conn: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    // Get the auth token
    let token = req
        .headers()
        .get("X-Scp-Auth")
        .ok_or_else(|| ErrorUnauthorized("Missing authentication token"))?
        .to_str()
        .map_err(ise!("GCEAT"))?;

    // Get roles for user
    let roles = crate::gaia_utils::get_roles(token)
        .await
        .map_err(ise!("GCGUR"))?;

    let is_admin = roles.contains("admin") || roles.contains("tutor");

    let challenges_and_services = challenge::Entity::find()
        .find_with_related(service::Entity)
        .all(conn.as_ref())
        .await
        .map_err(ise!("GCQCM"))?;

    let challenges_and_flags = challenge::Entity::find()
        .find_with_related(flag::Entity)
        .all(conn.as_ref())
        .await
        .map_err(ise!("GCQCF"))?;

    let categories: HashMap<i64, String> = category::Entity::find()
        .all(conn.as_ref())
        .await
        .map_err(ise!("GCQAC"))?
        .into_iter()
        .map(|c| (c.id, c.name))
        .collect();

    let mut map: HashMap<i64, (Vec<ReturnService>, Vec<ReturnFlag>)> = challenges_and_services
        .into_iter()
        .filter_map(|(challenge, services)| {
            let services = services
                .into_iter()
                .filter_map(|s| {
                    if !is_admin {
                        if let Some(dt) = s.not_before {
                            if chrono::offset::Utc::now().lt(&dt) {
                                return None;
                            }
                        }
                    }

                    Some(ReturnService {
                        category: categories.get(&s.category_id).unwrap().clone(),
                        id: s.id,
                        name: s.name,
                        not_after: s.not_after,
                        not_before: s.not_before,
                    })
                })
                .collect::<Vec<ReturnService>>();

            if services.is_empty() {
                return None;
            }

            Some((challenge.id, (services, vec![])))
        })
        .collect();

    for (challenge, flags) in challenges_and_flags {
        if map.contains_key(&challenge.id) {
            let flags = flags
                .into_iter()
                .map(|f| ReturnFlag {
                    category: categories.get(&f.category_id).unwrap().clone(),
                    display_name: f.display_name,
                    flag_type: f.flag_type,
                    id: f.id,
                    points: f.points,
                })
                .collect();
            map.insert(
                challenge.id,
                (map.get(&challenge.id).unwrap().0.clone(), flags),
            );
        }
    }

    let return_data: Vec<ReturnPayload> = map
        .into_iter()
        .map(|(k, v)| ReturnPayload {
            id: k,
            flags: v.1,
            services: v.0,
        })
        .collect();

    Ok(HttpResponse::Ok().json(return_data))
}
