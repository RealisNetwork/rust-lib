// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct JsTestsDragocatsBattleProcessEndpointsStartTestParams { # [serde (rename = "startGames")] pub start_games : f64 , # [serde (rename = "durationMinutes")] pub duration_minutes : f64 , # [serde (rename = "intervalMs")] pub interval_ms : f64 } impl Schema for JsTestsDragocatsBattleProcessEndpointsStartTestParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"startGames\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"durationMinutes\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intervalMs\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"intervalMs\",\"startGames\",\"durationMinutes\"]}") . unwrap () } } impl Agent for JsTestsDragocatsBattleProcessEndpointsStartTestParams { fn topic () -> & 'static str { "js-tests_dragocatsBattleProcessEndpoints_startTest" } fn method () -> & 'static str { "dragocatsBattleProcessEndpoints_startTest" } fn agent () -> & 'static str { "js-tests" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for JsTestsDragocatsBattleProcessEndpointsStartTestReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (JsTestsDragocatsBattleProcessEndpointsStartTestReturns) } } # [derive (Debug , Clone , Serialize)] pub struct JsTestsDragocatsBattleProcessEndpointsStartTestReturns ; impl Schema for JsTestsDragocatsBattleProcessEndpointsStartTestReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for JsTestsDragocatsBattleProcessEndpointsStartTestReturns { fn topic () -> & 'static str { "js-tests_dragocatsBattleProcessEndpoints_startTest" } fn method () -> & 'static str { "dragocatsBattleProcessEndpoints_startTest" } fn agent () -> & 'static str { "js-tests" } fn access_level () -> AccessLevel { } }