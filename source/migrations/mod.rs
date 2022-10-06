//! Database migrations.

use sea_orm_migration::prelude::*;

mod m20221004_000001_initial_setup;

/// [`sea_orm_migration`] struct, see
/// [Migration (API)](https://www.sea-ql.org/sea-orm-tutorial/ch01-03-migration-api.html)
/// for details.
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20221004_000001_initial_setup::Migration)]
  }
}
