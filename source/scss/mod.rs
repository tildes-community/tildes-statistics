//! All SCSS files.

use {
  async_std::{
    fs::{create_dir_all, write},
    path::PathBuf,
  },
  color_eyre::{eyre::Context, Result},
};

const MODERN_NORMALIZE_CSS: &str =
  include_str!("../../node_modules/modern-normalize/modern-normalize.css");

/// Generate the CSS files and write them.
pub async fn generate_css(parent: &PathBuf) -> Result<()> {
  let parent = parent.join("css");
  create_dir_all(&parent).await?;

  let render = |scss: &str| -> Result<String> {
    grass::from_string(scss.to_string(), &grass::Options::default())
      .wrap_err("Failed SCSS render")
  };

  let css_to_create = vec![
    ("modern-normalize.css", MODERN_NORMALIZE_CSS, false),
    ("common.css", include_str!("common.scss"), true),
    ("index.css", include_str!("index.scss"), true),
  ];

  for (file, css, is_scss) in css_to_create {
    let path = parent.join(file);
    if is_scss {
      write(path, render(css)?).await?;
    } else {
      write(path, css).await?;
    }
  }

  Ok(())
}
