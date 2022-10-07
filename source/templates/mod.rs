//! All HTML templates.

use {
  askama::Template,
  async_std::{fs::write, path::PathBuf},
  chrono::NaiveDate,
  color_eyre::Result,
};

use crate::utilities::today;

/// The template for the home page.
#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
  /// The string for the `<title>` element.
  pub page_title: String,

  /// The date of today's snapshot.
  pub today: NaiveDate,

  /// The user count from the group with the most subscribers.
  pub user_count: String,
}

impl HomeTemplate {
  /// Create a new [`HomeTemplate`].
  pub fn new(user_count: Option<i64>) -> Self {
    Self {
      page_title: "Tildes Statistics".to_string(),
      today: today(),
      user_count: user_count
        .map(|n| n.to_string())
        .unwrap_or_else(|| "unknown".to_string()),
    }
  }

  /// Render the template and write it to file.
  pub async fn render_to_file(&self, parent: &PathBuf) -> Result<()> {
    write(parent.join("index.html"), self.render()?).await?;

    Ok(())
  }
}
