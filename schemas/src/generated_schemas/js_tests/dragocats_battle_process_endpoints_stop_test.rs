// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for JsTestsDragocatsBattleProcessEndpointsStopTestParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(JsTestsDragocatsBattleProcessEndpointsStopTestParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct JsTestsDragocatsBattleProcessEndpointsStopTestParams;
impl Schema for JsTestsDragocatsBattleProcessEndpointsStopTestParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for JsTestsDragocatsBattleProcessEndpointsStopTestParams {
    fn topic() -> &'static str {
        "js-tests_dragocatsBattleProcessEndpoints_stopTest"
    }
    fn method() -> &'static str {
        "dragocatsBattleProcessEndpoints_stopTest"
    }
    fn agent() -> &'static str {
        "js-tests"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
impl<'de> Deserialize<'de> for JsTestsDragocatsBattleProcessEndpointsStopTestReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(JsTestsDragocatsBattleProcessEndpointsStopTestReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct JsTestsDragocatsBattleProcessEndpointsStopTestReturns;
impl Schema for JsTestsDragocatsBattleProcessEndpointsStopTestReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for JsTestsDragocatsBattleProcessEndpointsStopTestReturns {
    fn topic() -> &'static str {
        "js-tests_dragocatsBattleProcessEndpoints_stopTest"
    }
    fn method() -> &'static str {
        "dragocatsBattleProcessEndpoints_stopTest"
    }
    fn agent() -> &'static str {
        "js-tests"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
