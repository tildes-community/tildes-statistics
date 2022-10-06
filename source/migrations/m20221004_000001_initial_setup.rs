//! The migration for initial setup.

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20221004_000001_initial_setup"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Snapshot::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Snapshot::Id)
              .big_integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(Snapshot::Date).date().not_null())
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(GroupData::Table)
          .if_not_exists()
          .foreign_key(
            ForeignKey::create()
              .from(GroupData::Table, GroupData::SnapshotId)
              .to(Snapshot::Table, Snapshot::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .col(
            ColumnDef::new(GroupData::Id)
              .big_integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(GroupData::Description).text())
          .col(ColumnDef::new(GroupData::Name).text().not_null())
          .col(
            ColumnDef::new(GroupData::SnapshotId)
              .big_integer()
              .not_null(),
          )
          .col(
            ColumnDef::new(GroupData::Subscribers)
              .big_integer()
              .not_null(),
          )
          .to_owned(),
      )
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(GroupData::Table).to_owned())
      .await?;

    manager
      .drop_table(Table::drop().table(Snapshot::Table).to_owned())
      .await?;

    Ok(())
  }
}

#[derive(Iden)]
enum Snapshot {
  Table,
  Id,
  Date,
}

#[derive(Iden)]
enum GroupData {
  Table,
  Id,
  SnapshotId,
  Name,
  Description,
  Subscribers,
}
