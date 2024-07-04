use std::fmt::Display;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FactorioVersions {
  // 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 1.0 or 1.1
  #[serde(rename = "0.13")]
  V0_13,
  #[serde(rename = "0.14")]
  V0_14,
  #[serde(rename = "0.15")]
  V0_15,
  #[serde(rename = "0.16")]
  V0_16,
  #[serde(rename = "0.17")]
  V0_17,
  #[serde(rename = "0.18")]
  V0_18,
  #[serde(rename = "1.0")]
  V1_0,
  #[serde(rename = "1.1")]
  V1_1,
  Other(String),
}

impl Display for FactorioVersions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FactorioVersions::V0_13 => write!(f, "0.13"),
      FactorioVersions::V0_14 => write!(f, "0.14"),
      FactorioVersions::V0_15 => write!(f, "0.15"),
      FactorioVersions::V0_16 => write!(f, "0.16"),
      FactorioVersions::V0_17 => write!(f, "0.17"),
      FactorioVersions::V0_18 => write!(f, "0.18"),
      FactorioVersions::V1_0 => write!(f, "1.0"),
      FactorioVersions::V1_1 => write!(f, "1.1"),
      FactorioVersions::Other(s) => write!(f, "{}", s),
    }
  }
}