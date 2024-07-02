use reqwest::Method;
use serde::Deserialize;
use serde_json::{from_value, Value};
use tracing::{debug, instrument};
use url::Url;
use urlencoding::encode;

use super::fmod::{FModFull, FModShort};

#[derive(Debug, Deserialize)]
pub struct Context {
  pub username: String,
  pub(crate) token: String,
}

impl Context {
  /// Creates a new Context instance by sending a POST request to the Factorio authentication server.
  ///
  /// # Arguments
  ///
  /// * `username` - The username for authentication.
  /// * `password` - The password for authentication.
  /// * `email_code` - The email authentication code.
  ///
  /// # Returns
  ///
  /// * `Result<Self, reqwest::Error>` - Returns a Result containing the created Context instance or a reqwest::Error.
  #[instrument]
  pub async fn new(username: String, password: String, email_code: Option<String>) -> Result<Self, reqwest::Error> {
    let req_url = Url::parse_with_params("https://auth.factorio.com/api-login",
    &[
      ("username", encode(&username).to_string().as_str()),
      ("password", encode(&password).to_string().as_str()),
      ("api_version", "4"),
      ("require_game_ownership", "true"),
    ]).unwrap();
    debug!("{}", req_url);

    let req = reqwest::Client::new()
      .request(Method::POST, req_url);

    let code: Value = if let Some(code) = email_code {
      req.header("email_code", code)
    } else {
      req
    }.send().await?.json().await?;

    if let Ok(token) = from_value::<Vec<String>>(code.clone()) {
      return Ok(Context {
        username,
        token: token.first().unwrap().clone(),
      });
    } else if let Ok(ctx) = from_value::<Context>(code.clone()) {
      return Ok(ctx)
    } else {
      panic!("Coud not login.")
    }
  }

  /// Fetches short information about a mod from the Factorio mods server.
  ///
  /// # Arguments
  ///
  /// * `mod_name` - The name of the mod.
  ///
  /// # Returns
  ///
  /// * `Result<FModShort, reqwest::Error>` - Returns a Result containing the short mod information or a reqwest::Error.
  pub async fn get_mod_info(&self, mod_name: &str) -> Result<FModShort, reqwest::Error> {
    reqwest::Client::new()
      .request(Method::GET, format!("https://mods.factorio.com/api/mods/{}", encode(mod_name)))
      .send().await.unwrap().json().await
  }

  /// Fetches full information about a mod from the Factorio mods server.
  ///
  /// # Arguments
  ///
  /// * `mod_name` - The name of the mod.
  ///
  /// # Returns
  ///
  /// * `Result<FModFull, reqwest::Error>` - Returns a Result containing the full mod information or a reqwest::Error.
  pub async fn get_mod_info_full(&self, mod_name: &str) -> Result<FModFull, reqwest::Error> {
    reqwest::Client::new()
      .request(Method::GET, format!("https://mods.factorio.com/api/mods/{}/full", encode(mod_name)))
      .send().await.unwrap().json().await
  }
}