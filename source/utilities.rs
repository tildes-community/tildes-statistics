//! Helper functions and miscellaneous utilities.

use std::time::Duration;

use {
  async_std::task::sleep,
  chrono::{NaiveDate, Utc},
  color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
  },
  sea_orm::{ConnectOptions, Database, DatabaseConnection},
  surf::{Client, Config},
  tildes_parser::Html,
};

/// Creates the SeaQL [`DatabaseConnection`].
pub async fn create_db(sql_logging: bool) -> Result<DatabaseConnection> {
  let database_url = get_env_var("DATABASE_URL")?;

  let mut connect_options = ConnectOptions::new(database_url);
  connect_options.sqlx_logging(sql_logging);

  Database::connect(connect_options)
    .await
    .wrap_err("Failed to connect to database")
}

/// Creates the HTTP [`Client`].
pub fn create_http_client() -> Result<Client> {
  let user_agent = get_env_var("USER_AGENT")?;
  let http: Client = Config::default()
    .add_header("User-Agent", user_agent)
    .map_err(|err| eyre!(err))?
    .try_into()?;

  Ok(http)
}

/// Shorthand to download a URL and parse it to [`Html`].
pub async fn download_html(
  http: &Client,
  url: impl AsRef<str>,
) -> Result<Html> {
  sleep(Duration::from_millis(500)).await;
  let html = http
    .get(url)
    .recv_string()
    .await
    .map_err(|err| eyre!(err))?;

  Ok(Html::parse_document(&html))
}

/// Shorthand for [`std::env::var`] with wrapped error message.
pub fn get_env_var(key: &str) -> Result<String> {
  std::env::var(key).wrap_err(key.to_string())
}

/// Create a [`NaiveDate`] for today.
pub fn today() -> NaiveDate {
  Utc::now().date().naive_utc()
}
