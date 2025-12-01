pub use sea_orm_migration::prelude::*;
mod m20220101_000001_create_stations;
mod m20251119_183129_stations_add_coords;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_stations::Migration),
            Box::new(m20251119_183129_stations_add_coords::Migration),
        ]
    }
}
