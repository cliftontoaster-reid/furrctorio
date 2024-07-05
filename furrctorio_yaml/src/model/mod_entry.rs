use furrctorio_core::prelude::{Context, FModFull, FModRelease, FModShort};
use semver::VersionReq;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigModEntry {
  pub name: String,
  pub version: VersionReq,
  pub enabled: bool,
}

impl ConfigModEntry {
  #[instrument]
  pub fn new(name: String, version: VersionReq, enabled: bool) -> Self {
    Self {
      name,
      version,
      enabled,
    }
  }

  #[instrument]
  pub async fn get_mod(&self, ctx: &Context) -> Result<FModShort, reqwest::Error> {
    debug!("Downloading short information for mod '{}'", &self.name);

    ctx.get_mod_info(&self.name).await
  }

  #[instrument]
  pub async fn get_mod_full(&self, ctx: &Context) -> Result<FModFull, reqwest::Error> {
    debug!("Downloading full information for mod '{}'", &self.name);

    ctx.get_mod_info_full(&self.name).await
  }

  pub async fn find_last_release(&self, ctx: &Context) -> Result<Option<FModRelease>, reqwest::Error> {
    let smod = self.get_mod(ctx).await?;
    if let Some(last) = smod.latest_release {
      return Ok(Some(last));        
    }

    let fmod = self.get_mod_full(ctx).await?;
    if fmod.releases.is_empty() {
      Ok(None)
    } else {
      let mut releases = fmod
        .releases
        .clone()
        .iter()
        .filter(|&r| r.match_version(&self.version))
        .cloned()
        .collect::<Vec<FModRelease>>();
      releases.sort();

      Ok(Some(releases.last().unwrap().clone()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use furrctorio_core::prelude::Context;
  use semver::VersionReq;

  #[tokio::test]
  async fn test_get_mod() {
    dotenv::dotenv().ok();
    
    let ctx = Context::new_from_env();

    let entry = ConfigModEntry::new("stdlib".to_string(), VersionReq::parse("*").unwrap(), true);

    let mod_info = entry.get_mod(&ctx).await.unwrap();

    assert_eq!(mod_info.name, "stdlib");
  }

  #[tokio::test]
  async fn test_get_mod_full() {
    dotenv::dotenv().ok();

    let ctx = Context::new_from_env();

    let entry = ConfigModEntry::new("stdlib".to_string(), VersionReq::parse("*").unwrap(), true);

    let mod_info = entry.get_mod_full(&ctx).await.unwrap();

    assert_eq!(mod_info.name, "stdlib");
  }

  #[tokio::test]
  async fn test_find_last_release() {
    dotenv::dotenv().ok();
    
    let ctx = Context::new_from_env();

    let entry = ConfigModEntry::new("stdlib".to_string(), VersionReq::parse("*").unwrap(), true);

    let releases = entry.find_last_release(&ctx).await.unwrap();

    assert!(releases.is_some());

    let rel = releases.unwrap();
    let binding = entry.get_mod(&ctx).await.unwrap();
    let max_version = binding.releases.iter().max().unwrap();

    assert_eq!(&rel, max_version);
  }
}