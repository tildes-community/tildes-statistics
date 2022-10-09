//! Code for creating a new snapshot.

use {
  color_eyre::Result,
  sea_orm::{prelude::*, ActiveValue::*, TransactionTrait},
  tildes_parser::{Group, GroupList},
  tracing::{debug, info},
};

use crate::{
  group_data::{GroupDataActiveModel, GroupDataEntity},
  snapshots::{SnapshotActiveModel, SnapshotModel},
  utilities::{create_http_client, download_html, get_base_url, today},
};

impl SnapshotModel {
  /// Create a snapshot for today.
  pub async fn create(db: &DatabaseConnection, force: bool) -> Result<()> {
    let base_url = get_base_url();
    let snapshot_date = today();
    match (force, Self::get_by_date(db, snapshot_date).await?) {
      (true, Some(existing)) => {
        info!("Removing existing snapshot {:?}", existing);
        existing.delete(db).await?;
      }

      (false, Some(existing)) => {
        info!("Snapshot for today already exists");
        info!("Use --force to override snapshot {:?}", existing);
        return Ok(());
      }

      (_, None) => (),
    };

    let transaction = db.begin().await?;
    let snapshot = SnapshotActiveModel {
      date: Set(snapshot_date),
      ..Default::default()
    }
    .insert(&transaction)
    .await?;

    info!("Scraping data for snapshot {:?}", snapshot);

    let http = create_http_client()?;
    let group_list = GroupList::from_html(
      &download_html(&http, format!("{}/groups", base_url)).await?,
    )?;

    let mut groups_to_insert = vec![];

    for summary in group_list.summaries {
      debug!(summary = ?summary);
      let group = Group::from_html(
        &download_html(&http, format!("{}/{}", base_url, summary.name)).await?,
      )?;

      debug!(group = ?group);
      groups_to_insert.push(GroupDataActiveModel {
        description: Set(group.description),
        name: Set(group.name),
        snapshot_id: Set(snapshot.id),
        subscribers: Set(group.subscribers.into()),
        ..Default::default()
      });
    }

    info!("Inserting {} groups", groups_to_insert.len());
    GroupDataEntity::insert_many(groups_to_insert)
      .exec(&transaction)
      .await?;

    transaction.commit().await?;

    Ok(())
  }
}
