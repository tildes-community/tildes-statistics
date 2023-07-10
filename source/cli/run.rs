//! All logic for running the CLI.

use {
  async_std::fs::{copy, create_dir_all},
  clap::Parser,
  color_eyre::Result,
  sea_orm_migration::MigratorTrait,
  tracing::info,
};

use crate::{
  assets::write_assets,
  charts::UserCountChart,
  cli::{
    Cli, MainSubcommands, MigrateSubcommands, SnapshotSubcommands,
    WebSubcommands,
  },
  group_data::GroupDataModel,
  migrations::Migrator,
  scss::generate_css,
  snapshots::SnapshotModel,
  templates::{GroupTemplate, HomeTemplate},
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
        SnapshotModel::create(&db, force).await?;
      }

      SnapshotSubcommands::List {} => {
        for snapshot in SnapshotModel::get_all(&db).await? {
          info!("Snapshot {snapshot:?}")
        }
      }

      SnapshotSubcommands::Show { date, id } => {
        let date = date.unwrap_or_else(today);

        let snapshot = match id {
          Some(id) => SnapshotModel::get_by_id(&db, id).await,
          None => SnapshotModel::get_by_date(&db, date).await,
        }?;

        let snapshot = match (snapshot, id) {
          (None, Some(id)) => {
            info!("No snapshot exists for id {id}");
            return Ok(());
          }
          (None, None) => {
            info!("No snapshot exists for date {date}");
            return Ok(());
          }
          (Some(snapshot), _) => snapshot,
        };

        info!("Snapshot {snapshot:?}");
        for group in GroupDataModel::get_all_by_snapshot(&db, &snapshot).await?
        {
          info!(
            id = group.id,
            name = group.name,
            subscribers = group.subscribers,
          );
        }
      }
    },

    MainSubcommands::Web {
      command: web_command,
    } => match web_command {
      WebSubcommands::Build { output } => {
        let (groups, user_count_group) =
          if let Some(snapshot) = SnapshotModel::get_most_recent(&db).await? {
            (
              GroupDataModel::get_all_by_snapshot(&db, &snapshot).await?,
              GroupDataModel::get_tildes_official_data(&db, &snapshot).await?,
            )
          } else {
            (vec![], None)
          };

        create_dir_all(&output).await?;

        for group in &groups {
          let chart = UserCountChart {
            groups: GroupDataModel::get_n_most_recent(&db, 31, &group.name)
              .await?,
          };
          chart
            .render(&output, &group.name, true, true, "charts")
            .await?;
          chart
            .render(&output, &group.name, true, false, "charts-untruncated")
            .await?;

          {
            let total_chart = UserCountChart {
              groups: GroupDataModel::get_all(&db, &group.name).await?,
            };

            total_chart
              .render(&output, &group.name, false, true, "charts-total")
              .await?;
            total_chart
              .render(
                &output,
                &group.name,
                false,
                false,
                "charts-total-untruncated",
              )
              .await?;
          }

          GroupTemplate::new(group.description.clone(), &group.name)
            .await
            .render_to_file(&output)
            .await?;
        }

        HomeTemplate::new(
          groups,
          user_count_group.as_ref().map(|group| group.subscribers),
        )
        .await
        .render_to_file(&output)
        .await?;
        generate_css(&output).await?;
        write_assets(&output).await?;

        if let Some(group) = user_count_group {
          for dir in ["charts", "charts-untruncated"] {
            let from_path =
              output.join(&format!("{}/user-count/{}.svg", dir, &group.name));
            let to_path = output.join(format!("{}/main-user-count.svg", dir));
            copy(from_path, to_path).await?;
          }
        }
      }
    },
  }

  Ok(())
}
