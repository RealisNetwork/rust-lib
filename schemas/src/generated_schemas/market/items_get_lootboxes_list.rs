// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for MarketItemsGetLootboxesListParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (MarketItemsGetLootboxesListParams) } } # [derive (Debug , Clone , Serialize)] pub struct MarketItemsGetLootboxesListParams ; impl Schema for MarketItemsGetLootboxesListParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for MarketItemsGetLootboxesListParams { fn topic () -> & 'static str { "market_items_getLootboxesList" } fn method () -> & 'static str { "items_getLootboxesList" } fn agent () -> & 'static str { "market" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketItemsGetLootboxesListReturnsProductParamsParams { # [serde (rename = "currencyType")] pub currency_type : String , # [serde (rename = "id")] pub id : i32 , # [serde (rename = "productType")] pub product_type : String , # [serde (rename = "price")] pub price : String , # [serde (rename = "type")] pub r#type : i32 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct MarketItemsGetLootboxesListReturns { # [serde (rename = "product")] pub product : Vec < MarketItemsGetLootboxesListReturnsProductParamsParams > } impl Schema for MarketItemsGetLootboxesListReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"product\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"currencyType\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"productType\":{\"type\":\"string\"},\"price\":{\"type\":\"string\"},\"type\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"type\",\"productType\",\"currencyType\",\"price\"]}}},\"required\":[\"product\"]}") } } impl Agent for MarketItemsGetLootboxesListReturns { fn topic () -> & 'static str { "market_items_getLootboxesList" } fn method () -> & 'static str { "items_getLootboxesList" } fn agent () -> & 'static str { "market" } }