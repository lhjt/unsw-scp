use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum FlagType {
    #[serde(rename = "static")]
    #[sea_orm(num_value = 0)]
    Static,
    #[serde(rename = "dynamic")]
    #[sea_orm(num_value = 1)]
    Dynamic,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "flags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique, indexed)]
    pub id: String,
    #[sea_orm(indexed)]
    pub challenge_id: i64,
    #[sea_orm(indexed)]
    pub category_id: i64,
    #[sea_orm(indexed)]
    pub flag: String,
    pub flag_type: FlagType,
    pub points: i32,
    pub display_name: String,
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
    #[sea_orm(has_many = "super::submission::Entity")]
    Submission,
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

impl Related<super::submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
