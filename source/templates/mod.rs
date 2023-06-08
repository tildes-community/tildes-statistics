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

use async_std::fs::create_dir_all;

use crate::{
  group_data::GroupDataModel,
  utilities::{get_base_url, today},
};

/// The template for the home page.
#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {
  /// The base URL for links to the Tildes instance.
  pub base_url: String,

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
      base_url: get_base_url(),
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

/// The template for group-specific pages.
#[derive(Template)]
#[template(path = "group.html")]
pub struct GroupTemplate {
  /// The base URL for links to the Tildes instance.
  pub base_url: String,

  /// Extra HTML to insert in the body.
  pub extra_body_html: String,

  /// Extra HTML to insert in the head.
  pub extra_head_html: String,

  /// The group name for this group.
  pub group_name: String,

  /// The string for the `<title>` element.
  pub page_title: String,

  /// The date of today's snapshot.
  pub today: NaiveDate,
}

impl GroupTemplate {
  /// Create a new [`GroupTemplate`].
  pub async fn new(group_name: &str) -> Self {
    let extra_body_html = read_to_string("extra-body.html").await;
    let extra_head_html = read_to_string("extra-head.html").await;

    Self {
      base_url: get_base_url(),
      extra_body_html: extra_body_html.unwrap_or_default(),
      extra_head_html: extra_head_html.unwrap_or_default(),
      group_name: group_name.to_string(),
      page_title: "Tildes Statistics".to_string(),
      today: today(),
    }
  }

  /// Render the template and write it to file.
  pub async fn render_to_file(&self, parent: &PathBuf) -> Result<()> {
    let output_dir = parent.join(&self.group_name);
    create_dir_all(&output_dir).await?;
    write(output_dir.join("index.html"), self.render()?).await?;

    Ok(())
  }
}
