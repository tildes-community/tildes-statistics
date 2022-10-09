//! Miscellaneous assets.

use {
  async_std::{fs::write, path::PathBuf},
  color_eyre::Result,
};

/// Write all misellaneous asset files to where they belong.
pub async fn write_assets(parent: &PathBuf) -> Result<()> {
  let logo = include_bytes!("tildes-statistics.png");
  write(parent.join("tildes-statistics.png"), logo).await?;

  Ok(())
}
