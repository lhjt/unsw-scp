use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "submissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique, indexed)]
    pub id: i64,
    pub user_id: i64,
    pub flag_id: i64,
    #[sea_orm(indexed)]
    pub submission_time: chrono::DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::flag::Entity",
        from = "Column::FlagId",
        to = "super::flag::Column::Id"
    )]
    Flag,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::flag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
