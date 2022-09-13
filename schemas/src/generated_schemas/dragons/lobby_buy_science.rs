// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbyBuyScienceParams { # [serde (rename = "sciencePurchaseKey")] pub science_purchase_key : String , # [serde (rename = "userId")] pub user_id : String } impl Schema for DragonsLobbyBuyScienceParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"sciencePurchaseKey\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"sciencePurchaseKey\"]}") } } impl Agent for DragonsLobbyBuyScienceParams { fn topic () -> & 'static str { "dragons_lobby_buyScience" } fn method () -> & 'static str { "lobby_buyScience" } fn agent () -> & 'static str { "dragons" } } impl < 'de > Deserialize < 'de > for DragonsLobbyBuyScienceReturns { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (DragonsLobbyBuyScienceReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyBuyScienceReturns ; impl Schema for DragonsLobbyBuyScienceReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyBuyScienceReturns { fn topic () -> & 'static str { "dragons_lobby_buyScience" } fn method () -> & 'static str { "lobby_buyScience" } fn agent () -> & 'static str { "dragons" } }