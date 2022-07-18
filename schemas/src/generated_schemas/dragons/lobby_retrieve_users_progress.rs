// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyRetrieveUsersProgressParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyRetrieveUsersProgressParams {
    fn schema() -> Value {
        todo!()
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyRetrieveUsersProgressReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyRetrieveUsersProgressReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyRetrieveUsersProgressReturns;
