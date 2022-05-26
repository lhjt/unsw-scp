use entity::{role, user};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220101_000002_create_table" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(role::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(role::Column::RoleId)
                            .integer()
                            .auto_increment()
                            .unique_key()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(role::Column::Name).string().not_null())
                    .col(ColumnDef::new(role::Column::UserId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("role_userid_user_id_fk")
                            .from_tbl(role::Entity)
                            .from_col(role::Column::UserId)
                            .to_tbl(user::Entity)
                            .to_col(user::Column::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}
