// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbySaveTutorialProgressParams {
    #[serde(rename = "dataObject")]
    pub data_object: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbySaveTutorialProgressParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"dataObject\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"dataObject\"]}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbySaveTutorialProgressReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbySaveTutorialProgressReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbySaveTutorialProgressReturns;
impl Schema for CatsLobbySaveTutorialProgressReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}