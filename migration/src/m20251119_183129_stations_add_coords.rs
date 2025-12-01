use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(Station::Table)
                .add_column(ColumnDef::new(Station::Latitude).decimal_len(10, 3))
                .add_column(ColumnDef::new(Station::Longitude).decimal_len(10, 3))
                .to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(Station::Table)
                .drop_column(Station::Latitude)
                .drop_column(Station::Longitude)
                .to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Station {
    Table,
    Latitude,
    Longitude,
}
