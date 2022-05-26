use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique, indexed)]
    pub id:   i64,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::service::Entity")]
    Service,
    #[sea_orm(has_many = "super::flag::Entity")]
    Flag,
}

impl Related<super::service::Entity> for Entity {
    fn to() -> RelationDef { Relation::Service.def() }
}

impl Related<super::flag::Entity> for Entity {
    fn to() -> RelationDef { Relation::Flag.def() }
}

impl ActiveModelBehavior for ActiveModel {}
