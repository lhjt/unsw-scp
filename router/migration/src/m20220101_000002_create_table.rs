use router_entity::user;
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
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await
    }
}
