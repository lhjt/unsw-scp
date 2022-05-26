use entity::role;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220101_000003_create_index" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx-roles-names")
                    .table(role::Entity)
                    .col(role::Column::Name)
                    .to_owned(),
            )
            .await
    }
}
