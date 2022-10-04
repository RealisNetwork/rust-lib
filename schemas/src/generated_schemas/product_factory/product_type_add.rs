// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeAddParamsParamsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeAddParams { # [serde (rename = "name")] pub name : String , # [serde (rename = "params")] pub params : ProductFactoryProductTypeAddParamsParamsParams , # [serde (rename = "dropChance")] pub drop_chance : f64 , # [serde (rename = "rarity")] pub rarity : f64 , # [serde (rename = "underType")] pub under_type : String , # [serde (rename = "isNFT")] pub is_nft : bool , # [serde (rename = "type")] pub r#type : String , # [serde (rename = "personalType")] pub personal_type : String } impl Schema for ProductFactoryProductTypeAddParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"params\":{\"type\":\"object\",\"properties\":{}},\"dropChance\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"rarity\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"underType\":{\"type\":\"string\"},\"isNFT\":{\"type\":\"boolean\"},\"type\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"}},\"required\":[\"isNFT\",\"name\",\"type\",\"underType\",\"personalType\",\"params\",\"rarity\",\"dropChance\"]}") . unwrap () } } impl Agent for ProductFactoryProductTypeAddParams { fn topic () -> & 'static str { "productFactory_productType_add" } fn method () -> & 'static str { "productType_add" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeAddReturns (pub bool) ; impl Schema for ProductFactoryProductTypeAddReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for ProductFactoryProductTypeAddReturns { fn topic () -> & 'static str { "productFactory_productType_add" } fn method () -> & 'static str { "productType_add" } fn agent () -> & 'static str { "productFactory" } fn access_level () -> AccessLevel { } }