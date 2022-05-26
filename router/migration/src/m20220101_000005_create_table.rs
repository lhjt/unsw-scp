use router_entity::{category, challenge, flag};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000005_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(flag::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(flag::Column::Id)
                            .string()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(flag::Column::Flag)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(flag::Column::ChallengeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(challenge::Entity, challenge::Column::Id)
                            .from_col(flag::Column::ChallengeId),
                    )
                    .col(
                        ColumnDef::new(flag::Column::CategoryId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(category::Entity, category::Column::Id)
                            .from_col(flag::Column::CategoryId),
                    )
                    .col(ColumnDef::new(flag::Column::FlagType).integer().not_null())
                    .col(ColumnDef::new(flag::Column::Points).integer().not_null())
                    .col(
                        ColumnDef::new(flag::Column::DisplayName)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
