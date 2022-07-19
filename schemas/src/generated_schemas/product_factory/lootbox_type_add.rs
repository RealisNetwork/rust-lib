// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryLootboxTypeAddParams { # [serde (rename = "type")] pub r#type : String , # [serde (rename = "name")] pub name : String , # [serde (rename = "lootboxId")] pub lootbox_id : i32 , # [serde (rename = "dropChanceMultiplier")] pub drop_chance_multiplier : i32 } impl Schema for ProductFactoryLootboxTypeAddParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"dropChanceMultiplier\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"name\",\"type\",\"lootboxId\",\"dropChanceMultiplier\"]}") } } impl Agent for ProductFactoryLootboxTypeAddParams { fn topic () -> & 'static str { "productFactory_lootboxType_add" } fn method () -> & 'static str { "lootboxType_add" } fn agent () -> & 'static str { "productFactory" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryLootboxTypeAddReturns (bool) ; impl Schema for ProductFactoryLootboxTypeAddReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for ProductFactoryLootboxTypeAddReturns { fn topic () -> & 'static str { "productFactory_lootboxType_add" } fn method () -> & 'static str { "lootboxType_add" } fn agent () -> & 'static str { "productFactory" } }