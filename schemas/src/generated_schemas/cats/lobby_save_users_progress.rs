// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CatsLobbySaveUsersProgressParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "dataObject")]
    pub data_object: String,
}
pub type CatsLobbySaveUsersProgressReturns = ();
