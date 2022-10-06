//! All logic for snapshots.

use {
  color_eyre::Result,
  sea_orm::{prelude::*, QueryOrder},
};

use crate::entities::snapshot;

mod create;

pub use create::create;

/// Get a snapshot for a given date.
pub async fn get_by_date(
  db: &DatabaseConnection,
  date: ChronoDate,
) -> Result<Option<snapshot::Model>> {
  let existing = snapshot::Entity::find()
    .filter(snapshot::Column::Date.eq(date))
    .order_by_desc(snapshot::Column::Date)
    .one(db)
    .await?;

  Ok(existing)
}

/// Get all snapshots.
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<snapshot::Model>> {
  let snapshots = snapshot::Entity::find().all(db).await?;

  Ok(snapshots)
}
