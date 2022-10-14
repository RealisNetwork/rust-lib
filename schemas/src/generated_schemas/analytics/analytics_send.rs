// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AnalyticsAnalyticsSendParams { # [serde (rename = "key" , deserialize_with = "deserialize_to_string")] pub key : String , # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String , # [serde (rename = "param")] pub param : Option < String > , # [serde (rename = "value")] pub value : Option < () > } impl Schema for AnalyticsAnalyticsSendParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"param\":{\"type\":\"string\"},\"value\":{}},\"required\":[\"key\",\"userId\"]}") . unwrap () } } impl Agent for AnalyticsAnalyticsSendParams { fn topic () -> & 'static str { "analytics_analytics_send" } fn method () -> & 'static str { "analytics_send" } fn agent () -> & 'static str { "analytics" } fn access_level () -> AccessLevel { AccessLevel :: Public } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct AnalyticsAnalyticsSendReturns { # [serde (rename = "isSuccess")] pub is_success : bool } impl Schema for AnalyticsAnalyticsSendReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isSuccess\":{\"type\":\"boolean\"}},\"required\":[\"isSuccess\"]}") } } impl Agent for AnalyticsAnalyticsSendReturns { fn topic () -> & 'static str { "analytics_analytics_send" } fn method () -> & 'static str { "analytics_send" } fn agent () -> & 'static str { "analytics" } fn access_level () -> AccessLevel { AccessLevel :: Public } }