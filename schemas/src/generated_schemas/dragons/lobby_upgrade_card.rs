// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct DragonsLobbyUpgradeCardParams {
    #[serde(rename = "cardId")]
    pub card_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
