// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperCollectNicknamesParams {
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
}
impl Schema for CatsAndDragonsWrapperCollectNicknamesParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userIds\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"userIds\",\"appId\"]}") . unwrap ()
    }
}
impl Agent for CatsAndDragonsWrapperCollectNicknamesParams {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_collectNicknames"
    }
    fn method() -> &'static str {
        "wrapper_collectNicknames"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsAndDragonsWrapperCollectNicknamesReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
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
impl Agent for CatsAndDragonsWrapperCollectNicknamesReturns {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_collectNicknames"
    }
    fn method() -> &'static str {
        "wrapper_collectNicknames"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
