use crate::error::Error;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use semver::{Version, VersionReq};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sha1::{Digest, Sha1};
use std::{fmt::Display, str::FromStr, sync::Arc};
use url::Url;

use super::context::Context;

/// Represents a Factorio mod, which can be either short or full.
pub enum FMod {
  Short(FModShort),
  Full(FModFull),
}

impl FMod {
  /// Returns the name of the mod.
  pub fn name(&self) -> &str {
    match self {
      Self::Short(m) => &m.name,
      Self::Full(m) => &m.name,
    }
  }

  pub fn short(&self) -> FModShort {
    match self {
      FMod::Short(m) => m.clone(),
      FMod::Full(m) => FModShort {
        name: m.name.clone(),
        owner: m.owner.clone(),
        title: m.title.clone(),
        summary: m.summary.clone(),
        category: m.category.clone(),
        thumbnail: m.thumbnail.clone(),
        downloads_count: m.downloads_count,
        latest_release: m.releases.first().cloned(),
        releases: m.releases.clone(),
      },
    }
  }

  pub async fn full(&self, ctx: &Context) -> FModFull {
    match self {
      Self::Full(m) => m.clone(),
      Self::Short(m) => ctx.get_mod_info_full(m.name.as_str()).await.unwrap(),
    }
  }
}
/// Represents a full Factorio mod with detailed information.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct FModFull {
  /// The total number of downloads for the mod.
  pub downloads_count: usize,

  /// The machine-readable ID string of the mod.
  pub name: String,

  /// The Factorio username of the mod's author.
  pub owner: String,

  /// A list of different versions of the mod available for download.
  /// This is only populated when using the namelist parameter.
  pub releases: Vec<FModRelease>,

  /// A shorter description of the mod.
  pub summary: String,

  /// The human-readable name of the mod.
  pub title: String,

  /// A single category describing the mod. See Mod_details_API#Category.
  pub category: String,

  /// The relative path to the thumbnail of the mod.
  /// For mods that have no thumbnail, this may be absent or default to "/assets/.thumb.png".
  /// Prepend "assets-mod.factorio.com" to get the full URL.
  pub thumbnail: Option<String>,

  /// A string describing the recent changes to the mod.
  pub changelog: String,

  /// The ISO 8601 timestamp for when the mod was created.
  pub created_at: DateTime<Utc>,

  /// A longer description of the mod, in text only format.
  pub description: Option<String>,

  /// A URL to the mod's source code.
  pub source_url: Option<Url>,

  /// Deprecated: Use source_url instead. A link to the mod's github project page.
  /// Can be blank (""), in which case just prepend "github.com/".
  #[deprecated = "Use [`FModFull::source_url`] instead, or just prepend \"github.com/\". Can be blank (\"\")."]
  pub github_path: String,

  /// Usually a URL to the mod's main project page, but can be any string.
  pub homepage: Option<Url>,

  /// A list of tag names that categorize the mod. See #Tags.
  pub tags: Vec<FModTag>,

  /// The license that applies to the mod. See #License.
  pub license: License,

  /// True if the mod is marked as deprecated by its owner. Absent when false.
  pub deprecated: Option<bool>,
}

/// Represents the license that applies to a Factorio mod.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct License {
  /// A short description of the license.
  pub description: String,

  /// The unique id of the license.
  pub id: String,

  /// The internal name of the license.
  pub name: String,

  /// The human-readable title of the license.
  pub title: String,

  /// Usually a URL to the full license text, but can be any string.
  pub url: Option<Url>,
}

/// Represents the tags that categorize a Factorio mod.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum FModTag {
  /// Transportation of the player, be it vehicles or teleporters.
  Transportation,
  /// Augmented or new ways of transporting materials - belts, inserters, pipes!
  Logistics,
  /// Trains are great, but what if they could do even more?
  Trains,
  /// New ways to deal with enemies, be it attack or defense.
  Combat,
  /// Armors or armor equipment.
  Armor,
  /// Changes to enemies or entirely new enemies to deal with.
  Enemies,
  /// Map generation and terrain modification.
  Environment,
  /// New Ores and resources as well as machines.
  Mining,
  /// Things related to oil and other fluids.
  Fluids,
  /// Related to roboports and logistic robots.
  LogisticNetwork,
  /// Entities which interact with the circuit network.
  CircuitNetwork,
  /// Furnaces, assembling machines, production chains.
  Manufacturing,
  /// Changes to power production and distribution.
  Power,
  /// More than just chests.
  Storage,
  /// Change blueprint behavior.
  Blueprints,
  /// Play it your way.
  Cheats,
}

/// Represents a short Factorio mod with basic information.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct FModShort {
  /// The latest version of the mod available for download.
  /// Absent when the namelist parameter is used.
  pub latest_release: Option<FModRelease>,

  /// The total number of downloads for the mod.
  pub downloads_count: usize,

  /// The machine-readable ID string of the mod.
  pub name: String,

  /// The Factorio username of the mod's author.
  pub owner: String,

  /// A list of different versions of the mod available for download.
  /// This is only populated when using the namelist parameter.
  #[serde(default)]
  pub releases: Vec<FModRelease>,

  /// A shorter description of the mod.
  pub summary: String,

  /// The human-readable name of the mod.
  pub title: String,

  /// A single category describing the mod. See Mod_details_API#Category.
  pub category: String,

  /// The relative path to the thumbnail of the mod.
  /// For mods that have no thumbnail, this may be absent or default to "/assets/.thumb.png".
  /// Prepend "assets-mod.factorio.com" to get the full URL.
  pub thumbnail: Option<String>,
}

/// Represents the category of a Factorio mod.
/// The category helps users to understand the purpose and scope of the mod.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub enum FModCategory {
  /// No category.
  #[serde(rename = "no-category")]
  #[default]
  NoCategory,
  /// Mods introducing new content into the game.
  Content,
  /// Large total conversion mods.
  Overhaul,
  /// Small changes concerning balance, gameplay, or graphics.
  Tweaks,
  /// Providing the player with new tools or adjusting the game interface, without fundamentally changing gameplay.
  Utilities,
  /// Scenarios, maps, and puzzles.
  Scenarios,
  /// Collections of mods with tweaks to make them work together.
  ModPacks,
  /// Translations for other mods.
  Localizations,
  /// Lua libraries for use by other mods and submods that are parts of a larger mod.
  Internal,
}

/// Represents a release of a Factorio mod.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FModRelease {
  /// The URL to download the mod release.
  pub download_url: String,
  /// The name of the file for the mod release.
  pub file_name: String,
  /// The JSON metadata for the mod release.
  pub info_json: InfoJSON,
  /// The ISO 8601 timestamp for when the mod release was released.
  pub released_at: DateTime<Utc>,
  /// The version of the mod release.
  pub version: VersionEncapsulate,
  /// The SHA1 hash of the mod release.
  pub sha1: String,
}

#[derive(Debug, Clone)]
pub enum VersionEncapsulate {
  Version(Version),
  String(String),
}

impl<'de> Deserialize<'de> for VersionEncapsulate {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = String::deserialize(deserializer)?;
    match Version::parse(&value) {
      Ok(version) => Ok(VersionEncapsulate::Version(version)),
      Err(_) => Ok(VersionEncapsulate::String(value)),
    }
  }
}

impl Serialize for VersionEncapsulate {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      VersionEncapsulate::Version(version) => version.to_string().serialize(serializer),
      VersionEncapsulate::String(string) => string.serialize(serializer),
    }
  }
}

impl Default for FModRelease {
  fn default() -> Self {
    Self {
      download_url: String::new(),
      file_name: String::new(),
      info_json: InfoJSON::default(),
      released_at: DateTime::from_str("2023-01-01T00:00:00Z").unwrap(),
      version: VersionEncapsulate::Version(Version::new(0, 0, 0)),
      sha1: String::new(),
    }
  }
}

impl FModRelease {
  pub async fn download(&self, ctx: Arc<Context>) -> Result<(Bytes, String), reqwest::Error> {
    let req_url = Url::parse_with_params(
      &format!("https://mods.factorio.com/{}", self.download_url),
      &[
        ("username", ctx.username.clone()),
        ("token", ctx.token.clone()),
      ],
    )
    .unwrap();

    reqwest::get(req_url)
      .await?
      .bytes()
      .await
      .map(|ok| (ok, self.file_name.clone()))
  }

  pub fn validate(&self, data: &Bytes) -> bool {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize()).to_lowercase() == self.sha1
  }

  pub fn match_version(&self, version_req: &VersionReq) -> bool {
    match &self.version {
      VersionEncapsulate::Version(version) => version_req.matches(version),
      VersionEncapsulate::String(version_str) => {
        if version_str.starts_with("0.0.") {
          let req =
            VersionReq::parse(&version_req.clone().to_string().replace("0.0.", "0.1.")).unwrap();

          req.matches(&Version::parse(&version_str.replace("0.0.", "0.1.")).unwrap())
        } else {
          panic!("VersionReq cannot be parsed")
        }
      }
    }
  }
}

/// Represents the JSON metadata for a Factorio mod release.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct InfoJSON {
  /// The name of the mod.
  pub name: Option<String>,
  /// The version of the mod.
  pub version: Option<Version>,
  /// The human-readable title of the mod.
  pub title: Option<String>,
  /// The author of the mod.
  pub author: Option<String>,
  /// The version of Factorio that the mod is compatible with.
  pub factorio_version: Option<String>,
  /// The list of dependencies for the mod.
  #[serde(default)]
  pub dependencies: Vec<FModDependecies>,
}

/// Represents a dependency for a Factorio mod.
#[derive(Debug, Default, Clone)]
pub struct FModDependecies {
  /// The name of the dependency.
  pub name: String,
  /// The required version of the dependency.
  pub required_version: Option<VersionReq>,
  /// The prefix for the dependency.
  pub preffix: FModPreffix,
}

impl<'de> Deserialize<'de> for FModDependecies {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    FromStr::from_str(&s).map_err(|e| de::Error::custom(format!("{:?}", e)))
  }
}

impl Serialize for FModDependecies {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match (self.required_version.clone(), self.preffix) {
      (Some(version), FModPreffix::Required) => {
        serializer.serialize_str(&format!("{} {}", self.name, version))
      }
      (Some(version), preffix) => {
        serializer.serialize_str(&format!("{} {} {}", preffix, self.name, version))
      }
      (None, FModPreffix::Required) => serializer.serialize_str(&self.name),
      (None, preffix) => serializer.serialize_str(&format!("{} {}", preffix, self.name)),
    }
  }
}

impl FromStr for FModDependecies {
  type Err = Error;

  /// Parses a string into a FModDependecies.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
      [prefix, name] => Ok(FModDependecies {
        preffix: prefix.parse::<FModPreffix>()?,
        name: name.to_string(),
        ..Default::default()
      }),
      [prefix, name, version1, version2] => {
        let vers = VersionReq::parse(&format!("{}{}", version1, version2));
        if vers.is_err() {
          return Err(Error::ParcingError(
            "Invalid dependecies format: ".to_string() + s,
          ));
        }
        Ok(FModDependecies {
          preffix: prefix.parse::<FModPreffix>()?,
          name: name.to_string(),
          required_version: vers.ok(),
        })
      }
      [name, version1, version2] => {
        let vers = VersionReq::parse(&format!("{}{}", version1, version2));
        if vers.is_err() {
          return Err(Error::ParcingError(
            "Invalid dependecies format: ".to_string() + s,
          ));
        }
        Ok(FModDependecies {
          name: name.to_string(),
          required_version: vers.ok(),
          ..Default::default()
        })
      }
      _ => Err(Error::ParcingError(
        "Invalid dependecies format: ".to_string() + s,
      )),
    }
  }
}

/// Represents the prefix for a Factorio mod dependency.
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Copy)]
pub enum FModPreffix {
  /// The dependency is required.
  #[default]
  Required,
  /// The dependency is incompatible.
  #[serde(rename = "!")]
  Incompatible,
  /// The dependency is optional.
  #[serde(rename = "?")]
  Optional,
  /// The dependency is hidden optional.
  #[serde(rename = "(?)")]
  HiddenOptional,
  /// The dependency is non-changing.
  #[serde(rename = "~")]
  NonChanging,
}

impl FromStr for FModPreffix {
  type Err = Error;

  /// Parses a string into a FModPreffix.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "!" => Ok(FModPreffix::Incompatible),
      "?" => Ok(FModPreffix::Optional),
      "(?)" => Ok(FModPreffix::HiddenOptional),
      "~" => Ok(FModPreffix::NonChanging),
      "" => Ok(FModPreffix::Required),
      _ => Err(Error::InvalidPreffix(s.to_string())),
    }
  }
}

impl Display for FModPreffix {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FModPreffix::Incompatible => write!(f, "!"),
      FModPreffix::Optional => write!(f, "?"),
      FModPreffix::HiddenOptional => write!(f, "(?)"),
      FModPreffix::NonChanging => write!(f, "~"),
      FModPreffix::Required => write!(f, ""),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use semver::VersionReq;
  use serde_json::from_str;

  #[test]
  fn test_fmod_dependencies_from_str() {
    let dep = FModDependecies::from_str("base >= 0.18.27").unwrap();
    assert_eq!(dep.name, "base");
    assert_eq!(
      dep.required_version,
      Some(VersionReq::parse(">=0.18.27").unwrap())
    );
    assert_eq!(dep.preffix, FModPreffix::Required);

    let dep = FModDependecies::from_str("! other_mod").unwrap();
    assert_eq!(dep.name, "other_mod");
    assert_eq!(dep.preffix, FModPreffix::Incompatible);

    let dep = FModDependecies::from_str("? optional_mod").unwrap();
    assert_eq!(dep.name, "optional_mod");
    assert_eq!(dep.preffix, FModPreffix::Optional);

    let dep = FModDependecies::from_str("(?) hidden_optional_mod < 8.1").unwrap();
    assert_eq!(dep.name, "hidden_optional_mod");
    assert_eq!(dep.preffix, FModPreffix::HiddenOptional);

    let dep = FModDependecies::from_str("~ non_changing_mod").unwrap();
    assert_eq!(dep.name, "non_changing_mod");
    assert_eq!(dep.preffix, FModPreffix::NonChanging);

    assert!(FModDependecies::from_str("invalid_format coco uwu").is_err());
  }

  #[test]
  fn test_fmod_preffix_from_str() {
    assert_eq!(
      FModPreffix::from_str("!").unwrap(),
      FModPreffix::Incompatible
    );
    assert_eq!(FModPreffix::from_str("?").unwrap(), FModPreffix::Optional);
    assert_eq!(
      FModPreffix::from_str("(?)").unwrap(),
      FModPreffix::HiddenOptional
    );
    assert_eq!(
      FModPreffix::from_str("~").unwrap(),
      FModPreffix::NonChanging
    );
    assert_eq!(FModPreffix::from_str("").unwrap(), FModPreffix::Required);
    assert!(FModPreffix::from_str("invalid").is_err());
  }

  #[tokio::test]
  async fn test_request_mod_info() {
    let res: String = String::from(
      r#"{
  "category": "content",
  "downloads_count": 15,
  "name": "015_like_infinite_research",
  "owner": "marshkip",
  "releases": [
    {
      "download_url": "/download/015_like_infinite_research/5a5f1ae6adcc441024d72e0e",
      "file_name": "015_like_infinite_research_0.1.0.zip",
      "info_json": {
        "factorio_version": "0.14"
      },
      "released_at": "2016-11-11T07:07:22.473000Z",
      "sha1": "7529aeeba5382daa08fc6c907924eb0783119a22",
      "version": "0.1.0"
    }
  ],
  "summary": "add (almost) infinite research like planned in 0.15. see friday fact #161. this mod add infinite research for upgrading robots, turrets, weapons and researching speed.",
  "thumbnail": "/assets/84109a73b35230d21599ed5939d01090329ee5b6.thumb.png",
  "title": "infinite research (0.15 like)"
}"#,
    );
    let fmod: FModShort = from_str(&res).unwrap();
    assert!(fmod.latest_release.is_none());
    assert_eq!(fmod.downloads_count, 15);
    assert_eq!(fmod.name, "015_like_infinite_research");
    assert_eq!(fmod.owner, "marshkip");
    assert_eq!(fmod.releases.len(), 1);
    assert_eq!(fmod.summary, "add (almost) infinite research like planned in 0.15. see friday fact #161. this mod add infinite research for upgrading robots, turrets, weapons and researching speed.");
    assert_eq!(
      fmod.thumbnail,
      Some("/assets/84109a73b35230d21599ed5939d01090329ee5b6.thumb.png".to_string())
    );
    assert_eq!(fmod.title, "infinite research (0.15 like)");
    assert_eq!(fmod.category, "content");
  }

  #[tokio::test]
  async fn test_download_mod() {
    let release: FModRelease = from_str(
      r#"{
  "download_url": "/download/flib/5ecac7e44d121d000cd77c76",
  "file_name": "flib_0.1.0.zip",
  "info_json": {
    "dependencies": [
      "base >= 0.18.19"
    ],
    "factorio_version": "0.18"
  },
  "released_at": "2020-05-24T19:15:48.520000Z",
  "sha1": "55f7bbcfc0c0e831008b57c321db509bf3a25285",
  "version": "0.1.0"
}"#,
    )
    .unwrap();
    dotenv::dotenv().ok();

    let ctx = Arc::new(Context::new_from_env());

    let res = release.download(ctx).await.unwrap();

    assert!(release.validate(&res.0));
    assert_eq!(res.1, "flib_0.1.0.zip");
  }
}
