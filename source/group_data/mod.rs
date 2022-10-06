//! All logic for group datas.

use {color_eyre::Result, sea_orm::prelude::*};

use crate::entities::{group_data, snapshot};

/// Get all group datas from a given snapshot.
pub async fn get_all_by_snapshot(
  db: &DatabaseConnection,
  snapshot: &snapshot::Model,
) -> Result<Vec<group_data::Model>> {
  let groups = snapshot.find_related(group_data::Entity).all(db).await?;
  Ok(groups)
}
