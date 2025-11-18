use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Station::Table)
                    .if_not_exists()
                    .col(pk_uuid(Station::Id))
                    .col(binary(Station::PubKeyDigest))
                    .col(string(Station::Name))
                    .col(json_binary(Station::Metadata))
                    .col(timestamp(Station::CreatedAt))
                    .col(timestamp(Station::UpdatedAt))
                    .col(timestamp_null(Station::DeletedAt))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Station::Table)
                    .col(Station::PubKeyDigest)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Station::Table)
                    .col(Station::CreatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Station::Table)
                    .col(Station::UpdatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Station::Table)
                    .col(Station::DeletedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Station::Table)
                    .col(Station::PubKeyDigest)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Station::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Station {
    Table,
    Id,
    PubKeyDigest,
    Name,
    Metadata,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
