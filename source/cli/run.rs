//! All logic for running the CLI.

use {
  clap::Parser, color_eyre::Result, sea_orm_migration::MigratorTrait,
  tracing::info,
};

use crate::{
  cli::{Cli, MainSubcommands, MigrateSubcommands, SnapshotSubcommands},
  group_data::get_all_by_snapshot,
  migrations::Migrator,
  snapshots::{self, get_by_date},
  utilities::{create_db, today},
};

/// Run the CLI.
pub async fn run() -> Result<()> {
  let cli = Cli::parse();
  let db = create_db(cli.sql_logging).await?;

  if !cli.no_migrate {
    Migrator::up(&db, None).await?;
  }

  match cli.command {
    MainSubcommands::Migrate {
      command: migrate_command,
    } => match migrate_command {
      MigrateSubcommands::Down { amount } => {
        Migrator::down(&db, Some(amount)).await?;
      }

      MigrateSubcommands::Status => {
        Migrator::status(&db).await?;
      }

      MigrateSubcommands::Up { amount } => {
        Migrator::up(&db, Some(amount)).await?;
      }
    },

    MainSubcommands::Snapshot {
      command: snapshot_command,
    } => match snapshot_command {
      SnapshotSubcommands::Create { force } => {
        snapshots::create(&db, force).await?;
      }

      SnapshotSubcommands::List {} => {
        for snapshot in snapshots::get_all(&db).await? {
          info!("Snapshot {snapshot:?}")
        }
      }

      SnapshotSubcommands::Show { date } => {
        let date = date.unwrap_or_else(today);
        let snapshot = if let Some(snapshot) = get_by_date(&db, date).await? {
          info!("Snapshot {snapshot:?}");
          snapshot
        } else {
          info!("No snapshot exists for {date}");
          return Ok(());
        };

        let groups = get_all_by_snapshot(&db, &snapshot).await?;
        for group in groups {
          info!(
            id = group.id,
            name = group.name,
            subscribers = group.subscribers,
          );
        }
      }
    },
  }

  Ok(())
}
