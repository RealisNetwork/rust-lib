// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct RefundItemsAddParams { # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "hashItemId")] pub hash_item_id : f64 } impl Schema for RefundItemsAddParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"hashItemId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"hashItemId\"]}") . unwrap () } } impl Agent for RefundItemsAddParams { fn topic () -> & 'static str { "refund_items_add" } fn method () -> & 'static str { "items_add" } fn agent () -> & 'static str { "refund" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct RefundItemsAddReturns (pub bool) ; impl Schema for RefundItemsAddReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for RefundItemsAddReturns { fn topic () -> & 'static str { "refund_items_add" } fn method () -> & 'static str { "items_add" } fn agent () -> & 'static str { "refund" } fn access_level () -> AccessLevel { } }