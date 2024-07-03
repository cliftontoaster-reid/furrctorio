use semver::Version;
use serde::{Deserialize, Serialize};
use std::{fs::create_dir_all, path::PathBuf};
use crate::model::mod_entry::ConfigModEntry;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct FurrConfig {
  pub(crate) metadata: Metadata,
  pub mods: Vec<ConfigModEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
  #[serde(rename = "_v")]
  pub version: Version,
  pub factorio_version: Option<Version>,
  pub factorio_mod_folder: Option<PathBuf>,
}

impl Default for Metadata {
  fn default() -> Self {
    Self {
      version: Version::new(0, 1, 0),
      factorio_version: None,
      factorio_mod_folder: dirs::home_dir()
        .map(|folder| {
          let f = folder.join(".factorio").join("mods");
          if !f.exists() {
            create_dir_all(&f).unwrap();
          }
          Some(f)
        })
        .unwrap_or(None),
    }
  }
}
