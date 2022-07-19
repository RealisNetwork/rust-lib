// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusConfigAddParams { # [serde (rename = "price")] pub price : String , # [serde (rename = "priceInLis")] pub price_in_lis : String , # [serde (rename = "maxCount")] pub max_count : String , # [serde (rename = "isAvailable")] pub is_available : bool , # [serde (rename = "membership")] pub membership : String , # [serde (rename = "multiplier")] pub multiplier : String } impl Schema for StatusConfigAddParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"price\":{\"type\":\"string\"},\"priceInLis\":{\"type\":\"string\"},\"maxCount\":{\"type\":\"string\"},\"isAvailable\":{\"type\":\"boolean\"},\"membership\":{\"type\":\"string\"},\"multiplier\":{\"type\":\"string\"}},\"required\":[\"membership\",\"price\",\"multiplier\",\"priceInLis\",\"maxCount\",\"isAvailable\"]}") } } impl Agent for StatusConfigAddParams { fn topic () -> & 'static str { "status_config_add" } fn method () -> & 'static str { "config_add" } fn agent () -> & 'static str { "status" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct StatusConfigAddReturns (bool) ; impl Schema for StatusConfigAddReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for StatusConfigAddReturns { fn topic () -> & 'static str { "status_config_add" } fn method () -> & 'static str { "config_add" } fn agent () -> & 'static str { "status" } }