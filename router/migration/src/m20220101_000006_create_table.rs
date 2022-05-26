use router_entity::{flag, submission, user};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str { "m20220101_000006_create_table" }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(submission::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(submission::Column::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(submission::Column::UserId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(user::Entity, user::Column::Id)
                            .from_col(submission::Column::UserId),
                    )
                    .col(
                        ColumnDef::new(submission::Column::FlagId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(flag::Entity, flag::Column::Id)
                            .from_col(submission::Column::FlagId),
                    )
                    .col(
                        ColumnDef::new(submission::Column::SubmissionTime)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
