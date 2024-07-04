use reqwest::Method;
use serde::Deserialize;
use serde_json::{from_value, Value};
use tracing::{debug, instrument};
use url::Url;
use urlencoding::encode;
use crate::constants::FactorioVersions;

use super::{
  fmod::{FModFull, FModShort},
  pagination::FModList,
};

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
  pub async fn new(
    username: String,
    password: String,
    email_code: Option<String>,
  ) -> Result<Self, reqwest::Error> {
    let req_url = Url::parse_with_params(
      "https://auth.factorio.com/api-login",
      &[
        ("username", encode(&username).to_string().as_str()),
        ("password", encode(&password).to_string().as_str()),
        ("api_version", "4"),
        ("require_game_ownership", "true"),
      ],
    )
    .unwrap();
    debug!("{}", req_url);

    let req = reqwest::Client::new().request(Method::POST, req_url);

    let code: Value = if let Some(code) = email_code {
      req.header("email_code", code)
    } else {
      req
    }
    .send()
    .await?
    .json()
    .await?;

    if let Ok(token) = from_value::<Vec<String>>(code.clone()) {
      return Ok(Context {
        username,
        token: token.first().unwrap().clone(),
      });
    } else if let Ok(ctx) = from_value::<Context>(code.clone()) {
      return Ok(ctx);
    } else {
      panic!("Coud not login.")
    }
  }

  /// Creates a new Context instance from environment variables.
  ///
  /// This function retrieves the Factorio username and token from the environment variables
  /// "FACTORIO_USERNAME" and "FACTORIO_TOKEN" respectively, and uses them to create a new Context instance.
  ///
  /// # Panics
  ///
  /// This function will panic if either the "FACTORIO_USERNAME" or "FACTORIO_TOKEN" environment variables
  /// are not set.
  ///
  /// # Returns
  ///
  /// * `Context` - Returns a new Context instance.
  pub fn new_from_env() -> Self {
    Context {
      // Retrieve the Factorio username from the environment variable "FACTORIO_USERNAME".
      // If the environment variable is not set, this will panic.
      username: std::env::var("FACTORIO_USERNAME")
        .expect("FACTORIO_USERNAME must be set in the environment"),

      // Retrieve the Factorio token from the environment variable "FACTORIO_TOKEN".
      // If the environment variable is not set, this will panic.
      token: std::env::var("FACTORIO_TOKEN")
        .expect("FACTORIO_TOKEN must be set in the environment"),
    }
  }

  /// Creates a new request builder with the specified method and URL.
  ///
  /// # Arguments
  ///
  /// * `method` - The HTTP method for the request.
  /// * `url` - The URL for the request.
  /// * `auth` - A boolean indicating whether authentication is required.
  ///
  /// # Returns
  ///
  /// * `Result<reqwest::RequestBuilder, url::ParseError>` - Returns a Result containing the request builder or a url::ParseError.
  fn get_request(
    &self,
    method: Method,
    url: &str,
    auth: bool,
    options: Option<Vec<(&str, &str)>>,
  ) -> Result<reqwest::RequestBuilder, url::ParseError> {
    let req_url = if auth {
      // If authentication is required, parse the URL with username and token as parameters.
      let mut parms = vec![("username", self.username.as_str()), ("token", self.token.as_str())];

      if let Some(opt) = options {
        for o in opt {
          parms.push(o);
        }
      }
      Url::parse_with_params(
        url,
        &parms,
      )?
    } else if let Some(opt) = options {
      Url::parse_with_params(url, &opt)?
    } else {
      // If authentication is not required, parse the URL as is.
      Url::parse(url)?
    };

    // Create a new request builder with the specified method and URL.
    let base = reqwest::Client::new().request(method, req_url);

    // Return the request builder.
    Ok(base)
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
    self
      .get_request(
        Method::GET,
        &format!("https://mods.factorio.com/api/mods/{}", encode(mod_name)),
        false,
        None,
      )
      .unwrap()
      .send()
      .await?
      .json()
      .await
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
    self
      .get_request(
        Method::GET,
        &format!(
          "https://mods.factorio.com/api/mods/{}/full",
          encode(mod_name)
        ),
        false,
        None,
      )
      .unwrap()
      .send()
      .await
      .unwrap()
      .json()
      .await
  }

  /// Fetches a list of mods from the Factorio mods server.
  ///
  /// # Arguments
  ///
  /// * `page` - The page number to fetch.
  /// * `factorio_version` - The Factorio version to filter mods by.
  ///
  /// # Returns
  ///
  /// * `Result<FModList, reqwest::Error>` - Returns a Result containing the list of mods or a reqwest::Error.
  pub async fn get_mods(
    &self,
    page: usize,
    factorio_version: Option<FactorioVersions>,
  ) -> Result<FModList, reqwest::Error> {
    // Convert the Factorio version to a string if it is provided.
    let fv_str = factorio_version.map(|fv| fv.to_string());

    // Create a vector of parameters for the request.
    let mut parms = fv_str.as_ref().map(|txt| vec![("factorio_version", txt.as_str())]).unwrap_or_default();

    // Convert the page number to a string and add it to the parameters.
    let page_str = page.to_string();
    parms.push(("page", page_str.as_str()));

    // Create a new request builder with the specified method, URL, and parameters.
    self
      .get_request(
        Method::GET,
        "https://mods.factorio.com/api/mods",
        false,
        Some(parms),
      )
      .unwrap()
      // Send the request and await the response.
      .send()
      .await?
      // Parse the response as JSON and await the result.
      .json()
      .await
  }
}
