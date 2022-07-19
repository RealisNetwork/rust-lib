// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperCollectNicknamesParams {
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
    #[serde(rename = "appId")]
    pub app_id: i64,
}
impl Schema for CatsAndDragonsWrapperCollectNicknamesParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userIds\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userIds\",\"appId\"]}")
    }
}
impl<'de> Deserialize<'de> for CatsAndDragonsWrapperCollectNicknamesReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsAndDragonsWrapperCollectNicknamesReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsAndDragonsWrapperCollectNicknamesReturns;
impl Schema for CatsAndDragonsWrapperCollectNicknamesReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
