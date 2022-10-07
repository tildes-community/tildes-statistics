//! All CLI-related code.

use {
  async_std::path::PathBuf,
  chrono::NaiveDate,
  clap::{Parser, Subcommand},
};

mod run;

pub use run::run;

/// The Clap Derive CLI struct.
#[derive(Debug, Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
  /// The CLI subcommand.
  #[command(subcommand)]
  pub command: MainSubcommands,

  /// Don't run pending migrations automatically.
  #[clap(long)]
  pub no_migrate: bool,

  /// Output SQL queries in logging.
  #[clap(long, global = true)]
  pub sql_logging: bool,
}

/// Main CLI subcommands.
#[derive(Debug, Subcommand)]
pub enum MainSubcommands {
  /// Database migrations.
  Migrate {
    /// Database migrations.
    #[command(subcommand)]
    command: MigrateSubcommands,
  },

  /// Snapshot management.
  Snapshot {
    /// Snapshot management.
    #[command(subcommand)]
    command: SnapshotSubcommands,
  },

  /// Website management.
  Web {
    /// Website management.
    #[command(subcommand)]
    command: WebSubcommands,
  },
}

/// Migrate subcommands.
#[derive(Debug, Subcommand)]
pub enum MigrateSubcommands {
  /// Rollback applied migrations.
  Down {
    /// How many migrations to rollback.
    #[clap(default_value = "1")]
    amount: u32,
  },

  /// View the status of all migrations.
  Status,

  /// Apply pending migrations.
  Up {
    /// How many migrations to apply.
    #[clap(default_value = "1")]
    amount: u32,
  },
}

/// Snapshot subcommands.
#[derive(Debug, Subcommand)]
pub enum SnapshotSubcommands {
  /// Create a snapshot for today.
  Create {
    /// If a snapshot for today already exists, remove it and remake it.
    #[clap(long)]
    force: bool,
  },

  /// List available snapshots.
  List {},

  /// Show a snapshot.
  Show {
    /// The date of the snapshot to show, defaults to today.
    #[clap(short, long)]
    date: Option<NaiveDate>,
  },
}

/// Website subcommands.
#[derive(Debug, Subcommand)]
pub enum WebSubcommands {
  /// Build the website.
  Build {
    /// The output directory for the website files.
    #[clap(short, long, default_value = "public")]
    output: PathBuf,
  },
}
