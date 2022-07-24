pub use sea_orm_migration::prelude::*;

mod m20220724_222924_create_tictactoe_table;
mod m20220724_224947_create_tictactoe_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220724_222924_create_tictactoe_table::Migration),
            Box::new(m20220724_224947_create_tictactoe_table::Migration),
        ]
    }
