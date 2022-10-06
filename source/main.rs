//! # Tildes Statistics
//!
//! > **Statistics for Tildes.net.**

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use {
  color_eyre::{install, Result},
  dotenvy::dotenv,
  tracing_subscriber::filter::{EnvFilter, LevelFilter},
};

pub mod cli;
pub mod group_data;
pub mod migrations;
pub mod snapshots;
pub mod templates;
pub mod utilities;

/// The entities code is auto-generated using `sea-orm-cli`. With a database
/// and `.env` file setup, run the following command.
///
/// ```
/// sea-orm-cli generate entity -o source/entities
/// ```
#[allow(missing_docs, clippy::derive_partial_eq_without_eq)]
pub mod entities;

/// The main function.
pub fn main() -> Result<()> {
  install()?;
  dotenv().ok();

  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();
  tracing_subscriber::fmt().with_env_filter(env_filter).init();

  async_std::task::block_on(async { cli::run().await })
}
