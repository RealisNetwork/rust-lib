// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbyPurchaseValidationParams { # [serde (rename = "purchaseToken" , deserialize_with = "deserialize_to_string")] pub purchase_token : String , # [serde (rename = "productId" , deserialize_with = "deserialize_to_string")] pub product_id : String , # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String , # [serde (rename = "storeId")] pub store_id : f64 } impl Schema for DragonsLobbyPurchaseValidationParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"purchaseToken\":{\"type\":\"string\"},\"productId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"storeId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"storeId\",\"productId\",\"purchaseToken\"]}") . unwrap () } } impl Agent for DragonsLobbyPurchaseValidationParams { fn topic () -> & 'static str { "dragons_lobby_purchaseValidation" } fn method () -> & 'static str { "lobby_purchaseValidation" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { AccessLevel :: Public } } impl < 'de > Deserialize < 'de > for DragonsLobbyPurchaseValidationReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragonsLobbyPurchaseValidationReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyPurchaseValidationReturns ; impl Schema for DragonsLobbyPurchaseValidationReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyPurchaseValidationReturns { fn topic () -> & 'static str { "dragons_lobby_purchaseValidation" } fn method () -> & 'static str { "lobby_purchaseValidation" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { AccessLevel :: Public } }