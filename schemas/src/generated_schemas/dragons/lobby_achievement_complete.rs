// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct DragonsLobbyAchievementCompleteParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "achievementKey")]
    pub achievement_key: String,
}
