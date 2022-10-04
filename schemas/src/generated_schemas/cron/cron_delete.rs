// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CronCronDeleteParams { # [serde (rename = "id")] pub id : f64 } impl Schema for CronCronDeleteParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap () } } impl Agent for CronCronDeleteParams { fn topic () -> & 'static str { "cron_cron_delete" } fn method () -> & 'static str { "cron_delete" } fn agent () -> & 'static str { "cron" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CronCronDeleteReturns (pub bool) ; impl Schema for CronCronDeleteReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for CronCronDeleteReturns { fn topic () -> & 'static str { "cron_cron_delete" } fn method () -> & 'static str { "cron_delete" } fn agent () -> & 'static str { "cron" } fn access_level () -> AccessLevel { } }