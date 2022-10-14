// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryLootboxTypeAddParams { # [serde (rename = "lootboxId")] pub lootbox_id : i32 , # [serde (rename = "type" , deserialize_with = "deserialize_to_string")] pub r#type : String , # [serde (rename = "dropChanceMultiplier")] pub drop_chance_multiplier : i32 , # [serde (rename = "name" , deserialize_with = "deserialize_to_string")] pub name : String } impl Schema for ProductFactoryLootboxTypeAddParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"lootboxId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"type\":{\"type\":\"string\"},\"dropChanceMultiplier\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"name\":{\"type\":\"string\"}},\"required\":[\"name\",\"type\",\"lootboxId\",\"dropChanceMultiplier\"]}") . unwrap () } } impl Agent for ProductFactoryLootboxTypeAddParams { fn topic () -> & 'static str { "productFactory_lootboxType_add" } fn method () -> & 'static str { "lootboxType_add" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { AccessLevel :: Private } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryLootboxTypeAddReturns (pub bool) ; impl Schema for ProductFactoryLootboxTypeAddReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for ProductFactoryLootboxTypeAddReturns { fn topic () -> & 'static str { "productFactory_lootboxType_add" } fn method () -> & 'static str { "lootboxType_add" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { AccessLevel :: Private } }