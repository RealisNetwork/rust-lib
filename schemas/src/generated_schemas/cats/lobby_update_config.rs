// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CatsLobbyUpdateConfigParams {
    #[serde(rename = "configKey")]
    pub config_key: String,
    #[serde(rename = "configJson")]
    pub config_json: String,
}
