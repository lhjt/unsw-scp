use router_entity::service;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000009_create_index"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .table(service::Entity)
                    .name("idx-services-notafter")
                    .col(service::Column::NotAfter)
                    .to_owned(),
            )
            .await
    }
}
