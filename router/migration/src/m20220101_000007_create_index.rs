use router_entity::service;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220101_000007_create_index" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .table(service::Entity)
                    .name("idx-services-externalhostname")
                    .col(service::Column::ExternalHostname)
                    .to_owned(),
            )
            .await
    }
}
