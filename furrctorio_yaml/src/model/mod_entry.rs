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
        .filter(|&r| self.version.matches(&r.version))
        .cloned()
        .collect::<Vec<FModRelease>>();

      releases.sort_by(|a, b| a.version.partial_cmp(&b.version).unwrap());

      Ok(Some(releases.last().unwrap().clone()))
    }
  }
}
