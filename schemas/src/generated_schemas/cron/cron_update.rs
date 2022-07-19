// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CronCronUpdateParams { # [serde (rename = "id")] pub id : i64 , # [serde (rename = "howOften")] pub how_often : i64 , # [serde (rename = "startsAt")] pub starts_at : i64 } impl Schema for CronCronUpdateParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"howOften\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"startsAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"howOften\",\"startsAt\"]}") } } impl Agent for CronCronUpdateParams { fn topic () -> & 'static str { "cron_cron_update" } fn method () -> & 'static str { "cron_update" } fn agent () -> & 'static str { "cron" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CronCronUpdateReturns (bool) ; impl Schema for CronCronUpdateReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for CronCronUpdateReturns { fn topic () -> & 'static str { "cron_cron_update" } fn method () -> & 'static str { "cron_update" } fn agent () -> & 'static str { "cron" } }