// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CronCronCreateParams { # [serde (rename = "key")] pub key : String , # [serde (rename = "howOften")] pub how_often : i64 , # [serde (rename = "startsAt")] pub starts_at : i64 } impl Schema for CronCronCreateParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"},\"howOften\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"startsAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"howOften\",\"startsAt\",\"key\"]}") } } impl Agent for CronCronCreateParams { fn topic () -> & 'static str { "cron_cron_create" } fn method () -> & 'static str { "cron_create" } fn agent () -> & 'static str { "cron" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CronCronCreateReturns (bool) ; impl Schema for CronCronCreateReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for CronCronCreateReturns { fn topic () -> & 'static str { "cron_cron_create" } fn method () -> & 'static str { "cron_create" } fn agent () -> & 'static str { "cron" } }