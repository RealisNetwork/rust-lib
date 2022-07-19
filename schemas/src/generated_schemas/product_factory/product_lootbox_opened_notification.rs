// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for ProductFactoryProductLootboxOpenedNotificationParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (ProductFactoryProductLootboxOpenedNotificationParams) } } # [derive (Debug , Clone , Serialize)] pub struct ProductFactoryProductLootboxOpenedNotificationParams ; impl Schema for ProductFactoryProductLootboxOpenedNotificationParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ProductFactoryProductLootboxOpenedNotificationParams { fn topic () -> & 'static str { "productFactory_product_lootboxOpenedNotification" } fn method () -> & 'static str { "product_lootboxOpenedNotification" } fn agent () -> & 'static str { "productFactory" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductLootboxOpenedNotificationReturnsRewardsParamsParams { # [serde (rename = "type")] pub r#type : i32 , # [serde (rename = "amount")] pub amount : i32 , # [serde (rename = "itemId")] pub item_id : i32 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryProductLootboxOpenedNotificationReturns { # [serde (rename = "rewards")] pub rewards : Vec < ProductFactoryProductLootboxOpenedNotificationReturnsRewardsParamsParams > , # [serde (rename = "bindingId")] pub binding_id : i32 } impl Schema for ProductFactoryProductLootboxOpenedNotificationReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"rewards\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"type\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"amount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"type\",\"amount\",\"itemId\"]}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"rewards\",\"bindingId\"]}") } } impl Agent for ProductFactoryProductLootboxOpenedNotificationReturns { fn topic () -> & 'static str { "productFactory_product_lootboxOpenedNotification" } fn method () -> & 'static str { "product_lootboxOpenedNotification" } fn agent () -> & 'static str { "productFactory" } }