// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeUpdateParamsParamsParams { } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeUpdateParams { # [serde (rename = "type")] pub r#type : Option < String > , # [serde (rename = "rarity")] pub rarity : Option < f64 > , # [serde (rename = "isNFT")] pub is_nft : Option < bool > , # [serde (rename = "underType")] pub under_type : Option < String > , # [serde (rename = "name")] pub name : Option < String > , # [serde (rename = "personalType")] pub personal_type : String , # [serde (rename = "dropChance")] pub drop_chance : Option < f64 > , # [serde (rename = "params")] pub params : Option < ProductFactoryProductTypeUpdateParamsParamsParams > } impl Schema for ProductFactoryProductTypeUpdateParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"string\"},\"rarity\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isNFT\":{\"type\":\"boolean\"},\"underType\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"dropChance\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"params\":{\"type\":\"object\",\"properties\":{},\"required\":null}},\"required\":[\"personalType\"]}") } } impl Agent for ProductFactoryProductTypeUpdateParams { fn topic () -> & 'static str { "productFactory_productType_update" } fn method () -> & 'static str { "productType_update" } fn agent () -> & 'static str { "productFactory" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductTypeUpdateReturns (pub bool) ; impl Schema for ProductFactoryProductTypeUpdateReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"boolean\"}") } } impl Agent for ProductFactoryProductTypeUpdateReturns { fn topic () -> & 'static str { "productFactory_productType_update" } fn method () -> & 'static str { "productType_update" } fn agent () -> & 'static str { "productFactory" } }