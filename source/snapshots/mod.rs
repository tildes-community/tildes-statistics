//! All logic for snapshots.

use {
  color_eyre::Result,
  sea_orm::{prelude::*, QueryOrder},
};

mod create;

pub use crate::entities::snapshot::{
  ActiveModel as SnapshotActiveModel, Column as SnapshotColumn,
  Entity as SnapshotEntity, Model as SnapshotModel,
};

impl SnapshotModel {
  /// Get a snapshot for a given date.
  pub async fn get_by_date(
    db: &DatabaseConnection,
    date: ChronoDate,
  ) -> Result<Option<Self>> {
    let existing = SnapshotEntity::find()
      .filter(SnapshotColumn::Date.eq(date))
      .order_by_desc(SnapshotColumn::Date)
      .one(db)
      .await?;

    Ok(existing)
  }

  /// Get a snapshot by a given ID.
  pub async fn get_by_id(
    db: &DatabaseConnection,
    id: i64,
  ) -> Result<Option<Self>> {
    let snapshot = SnapshotEntity::find()
      .filter(SnapshotColumn::Id.eq(id))
      .one(db)
      .await?;

    Ok(snapshot)
  }

  /// Get all snapshots.
  pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Self>> {
    let snapshots = SnapshotEntity::find().all(db).await?;

    Ok(snapshots)
  }

  /// Get the most recent snapshot.
  pub async fn get_most_recent(
    db: &DatabaseConnection,
  ) -> Result<Option<Self>> {
    let snapshot = SnapshotEntity::find()
      .order_by_desc(SnapshotColumn::Date)
      .one(db)
      .await?;

    Ok(snapshot)
  }
}
