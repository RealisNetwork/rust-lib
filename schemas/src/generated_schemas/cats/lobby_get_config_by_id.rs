// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CatsLobbyGetConfigByIdParams {
    #[serde(rename = "configKey")]
    pub config_key: String,
}
pub type CatsLobbyGetConfigByIdReturns = ();
