use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
  ParcingError(String),
  InvalidPreffix(String),
  IoError(std::io::Error),
  APIError(APIError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
  pub message: String,
}

impl std::fmt::Display for APIError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for APIError {}
