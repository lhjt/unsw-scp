use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "services")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique, indexed)]
    pub id: i64,
    #[sea_orm(indexed)]
    pub challenge_id: i64,
    #[sea_orm(indexed)]
    pub category_id: i64,
    pub name: String,
    #[sea_orm(unique, indexed)]
    pub internal_hostname: String,
    #[sea_orm(indexed)]
    pub external_hostname: String,
    /// Not before: time before which the service is inaccessible to students.
    #[sea_orm(indexed)]
    pub not_before: chrono::DateTime<Utc>,
    /// Not after: time after which the service is inaccessible to students.
    #[sea_orm(indexed)]
    pub not_after: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::challenge::Entity",
        from = "Column::ChallengeId",
        to = "super::challenge::Column::Id"
    )]
    Challenge,
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
}

impl Related<super::challenge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Challenge.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
