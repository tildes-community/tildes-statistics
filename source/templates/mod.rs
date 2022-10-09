//! All HTML templates.

use {
  askama::Template,
  async_std::{
    fs::{read_to_string, write},
    path::PathBuf,
  },
  chrono::NaiveDate,
  color_eyre::Result,
};

use crate::{group_data::GroupDataModel, utilities::today};

/// The template for the home page.
#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
  /// Extra HTML to insert in the body.
  pub extra_body_html: String,

  /// Extra HTML to insert in the head.
  pub extra_head_html: String,

  /// The groups to create the table with.
  pub groups: Vec<GroupDataModel>,

  /// The string for the `<title>` element.
  pub page_title: String,

  /// The date of today's snapshot.
  pub today: NaiveDate,

  /// The user count from the group with the most subscribers.
  pub user_count: String,
}

impl HomeTemplate {
  /// Create a new [`HomeTemplate`].
  pub async fn new(
    groups: Vec<GroupDataModel>,
    user_count: Option<i64>,
  ) -> Self {
    let extra_body_html = read_to_string("extra-body.html").await;
    let extra_head_html = read_to_string("extra-head.html").await;

    Self {
      extra_body_html: extra_body_html.unwrap_or_default(),
      extra_head_html: extra_head_html.unwrap_or_default(),
      groups,
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
