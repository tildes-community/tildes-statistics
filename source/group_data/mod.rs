//! All logic for group datas.

use {
  color_eyre::Result,
  sea_orm::{prelude::*, QueryOrder, QuerySelect},
};

pub use crate::{
  entities::group_data::{
    ActiveModel as GroupDataActiveModel, Column as GroupDataColumn,
    Entity as GroupDataEntity, Model as GroupDataModel,
  },
  snapshots::SnapshotModel,
};

impl GroupDataModel {
  /// Get all group datas from a given snapshot.
  pub async fn get_all_by_snapshot(
    db: &DatabaseConnection,
    snapshot: &SnapshotModel,
  ) -> Result<Vec<Self>> {
    let groups = snapshot.find_related(GroupDataEntity).all(db).await?;
    Ok(groups)
  }

  /// Get the group with the highest subscriber count from a given snapshot.
  pub async fn get_highest_subscribers(
    db: &DatabaseConnection,
    snapshot: &SnapshotModel,
  ) -> Result<Option<Self>> {
    let group = snapshot
      .find_related(GroupDataEntity)
      .order_by_desc(GroupDataColumn::Subscribers)
      .one(db)
      .await?;

    Ok(group)
  }

  /// Get the N most recently saved group datas from a given group name.
  pub async fn get_n_most_recent(
    db: &DatabaseConnection,
    amount: u64,
    name: &str,
  ) -> Result<Vec<Self>> {
    let groups = GroupDataEntity::find()
      .order_by_asc(GroupDataColumn::SnapshotId)
      .filter(GroupDataColumn::Name.eq(name))
      .limit(amount)
      .all(db)
      .await?;

    Ok(groups)
  }
}
