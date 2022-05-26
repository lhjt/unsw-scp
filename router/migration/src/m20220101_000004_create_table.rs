use router_entity::{category, challenge, service};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220101_000004_create_table" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(service::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(service::Column::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(service::Column::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(service::Column::ChallengeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(challenge::Entity, challenge::Column::Id)
                            .from_col(service::Column::ChallengeId),
                    )
                    .col(
                        ColumnDef::new(service::Column::CategoryId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(category::Entity, category::Column::Id)
                            .from_col(service::Column::CategoryId),
                    )
                    .col(
                        ColumnDef::new(service::Column::InternalHostname)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(service::Column::ExternalHostname)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(service::Column::NotBefore).timestamp())
                    .col(ColumnDef::new(service::Column::NotAfter).timestamp())
                    .to_owned(),
            )
            .await
    }
}
