// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for JsTestsDragocatsBattleProcessEndpointsGetTestStateParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(JsTestsDragocatsBattleProcessEndpointsGetTestStateParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct JsTestsDragocatsBattleProcessEndpointsGetTestStateParams;
impl Schema for JsTestsDragocatsBattleProcessEndpointsGetTestStateParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for JsTestsDragocatsBattleProcessEndpointsGetTestStateParams {
    fn topic() -> &'static str {
        "js-tests_dragocatsBattleProcessEndpoints_getTestState"
    }
    fn method() -> &'static str {
        "dragocatsBattleProcessEndpoints_getTestState"
    }
    fn agent() -> &'static str {
        "js-tests"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsTestsDragocatsBattleProcessEndpointsGetTestStateReturns {
    #[serde(rename = "intervalMs")]
    pub interval_ms: f64,
    #[serde(rename = "startGames")]
    pub start_games: f64,
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: f64,
}
impl Schema for JsTestsDragocatsBattleProcessEndpointsGetTestStateReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"intervalMs\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"startGames\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"durationMinutes\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"intervalMs\",\"startGames\",\"durationMinutes\"]}")
    }
}
impl Agent for JsTestsDragocatsBattleProcessEndpointsGetTestStateReturns {
    fn topic() -> &'static str {
        "js-tests_dragocatsBattleProcessEndpoints_getTestState"
    }
    fn method() -> &'static str {
        "dragocatsBattleProcessEndpoints_getTestState"
    }
    fn agent() -> &'static str {
        "js-tests"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
