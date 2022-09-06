// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsTestsDragocatsBattleProcessEndpointsStartTestParams {
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: f64,
    #[serde(rename = "startGames")]
    pub start_games: f64,
    #[serde(rename = "intervalMs")]
    pub interval_ms: f64,
}
impl Schema for JsTestsDragocatsBattleProcessEndpointsStartTestParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"durationMinutes\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"startGames\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intervalMs\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"intervalMs\",\"startGames\",\"durationMinutes\"]}")
    }
}
impl Agent for JsTestsDragocatsBattleProcessEndpointsStartTestParams {
    fn topic() -> &'static str {
        "js-tests_dragocatsBattleProcessEndpoints_startTest"
    }
    fn method() -> &'static str {
        "dragocatsBattleProcessEndpoints_startTest"
    }
    fn agent() -> &'static str {
        "js-tests"
    }
}
impl<'de> Deserialize<'de> for JsTestsDragocatsBattleProcessEndpointsStartTestReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(JsTestsDragocatsBattleProcessEndpointsStartTestReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct JsTestsDragocatsBattleProcessEndpointsStartTestReturns;
impl Schema for JsTestsDragocatsBattleProcessEndpointsStartTestReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for JsTestsDragocatsBattleProcessEndpointsStartTestReturns {
    fn topic() -> &'static str {
        "js-tests_dragocatsBattleProcessEndpoints_startTest"
    }
    fn method() -> &'static str {
        "dragocatsBattleProcessEndpoints_startTest"
    }
    fn agent() -> &'static str {
        "js-tests"
    }
}
