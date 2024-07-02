use super::{context::Context, fmod::{FModFull, FModShort}};
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModList {
  pub mods: Vec<ModEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModEntry {
  pub name: String,
  pub enabled: bool,
}

impl ModList {
  /// This function returns a vector of `FModShort` objects which contain short information about each mod.
  ///
  /// # Arguments
  ///
  /// * `ctx` - A reference to an `Arc<Context>` object which is used to get the mod information.
  ///
  /// # Returns
  ///
  /// * `Vec<FModShort>` - A vector of `FModShort` objects which contain short information about each mod.
  pub async fn get_mods_info(&self, ctx: &Arc<Context>) -> Vec<FModShort> {
    stream::iter(&self.mods)
      .then(|m| ctx.get_mod_info(&m.name))
      .map(|i| i.unwrap())
      .collect()
      .await
  }

  /// This function returns a vector of `FModFull` objects which contain full information about each mod.
  ///
  /// # Arguments
  ///
  /// * `ctx` - A reference to an `Arc<Context>` object which is used to get the mod information.
  ///
  /// # Returns
  ///
  /// * `Vec<FModFull>` - A vector of `FModFull` objects which contain full information about each mod.
  pub async fn get_mods_info_full(&self, ctx: &Arc<Context>) -> Vec<FModFull> {
    stream::iter(&self.mods)
      .then(|m| ctx.get_mod_info_full(&m.name))
      .map(|i| i.unwrap())
      .collect()
      .await
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::context::Context;
  use std::sync::Arc;

  #[tokio::test]
  async fn test_get_mods_info() {
    let ctx = Arc::new(Context {
      token: "".to_string(),
      username: "".to_string(),
    });
    let mlist = ModList {
      mods: vec![
        ModEntry {
          name: "fcpu".to_string(),
          enabled: true,
        },
        ModEntry {
          name: "flib".to_string(),
          enabled: true,
        },
        ModEntry {
          name: "helmod".to_string(),
          enabled: true,
        }
      ],
    };

    let mods = mlist.get_mods_info(&ctx).await;
    assert_eq!(mods.len(), 3);
    let names = mods.iter().map(|m| m.name.clone()).collect::<Vec<String>>();
    assert!(names.contains(&"fcpu".to_string()));
    assert!(names.contains(&"flib".to_string()));
    assert!(names.contains(&"helmod".to_string()));
  }

  #[tokio::test]
  async fn test_get_mods_info_full() {
    let ctx = Arc::new(Context {
      token: "".to_string(),
      username: "".to_string(),
    });
    let mlist = ModList {
      mods: vec![
        ModEntry {
          name: "RealisticReactorGlow".to_string(),
          enabled: true,
        },
        ModEntry {
          name: "RealisticReactors".to_string(),
          enabled: true,
        },
        ModEntry {
          name: "stdlib".to_string(),
          enabled: true,
        }
      ],
    };

    let mods = mlist.get_mods_info(&ctx).await;
    assert_eq!(mods.len(), 3);
    let names = mods.iter().map(|m| m.name.clone()).collect::<Vec<String>>();
    assert!(names.contains(&"RealisticReactorGlow".to_string()));
    assert!(names.contains(&"RealisticReactors".to_string()));
    assert!(names.contains(&"stdlib".to_string()));
  }
}