// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for ProductFactoryLootboxTypeGetAllParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (ProductFactoryLootboxTypeGetAllParams) } } # [derive (Debug , Clone , Serialize)] pub struct ProductFactoryLootboxTypeGetAllParams ; impl Schema for ProductFactoryLootboxTypeGetAllParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ProductFactoryLootboxTypeGetAllParams { fn topic () -> & 'static str { "productFactory_lootboxType_getAll" } fn method () -> & 'static str { "lootboxType_getAll" } fn agent () -> & 'static str { "productFactory" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryLootboxTypeGetAllReturnsParams { # [serde (rename = "dropChanceMultiplier")] pub drop_chance_multiplier : i32 , # [serde (rename = "name")] pub name : String , # [serde (rename = "lootboxId")] pub lootbox_id : i32 , # [serde (rename = "id")] pub id : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryLootboxTypeGetAllReturns (Vec < ProductFactoryLootboxTypeGetAllReturnsParams >) ; impl Schema for ProductFactoryLootboxTypeGetAllReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"dropChanceMultiplier\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"name\":{\"type\":\"string\"},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"name\",\"lootboxId\",\"dropChanceMultiplier\"]}}") } } impl Agent for ProductFactoryLootboxTypeGetAllReturns { fn topic () -> & 'static str { "productFactory_lootboxType_getAll" } fn method () -> & 'static str { "lootboxType_getAll" } fn agent () -> & 'static str { "productFactory" } }