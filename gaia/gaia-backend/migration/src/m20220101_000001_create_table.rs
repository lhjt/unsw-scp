use entity::user;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::UserId)
                            .string()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(user::Column::Name).string())
                    .col(ColumnDef::new(user::Column::Email).string().not_null())
                    .to_owned(),
            )
            .await
    }
}
