// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbyBuyCatParams { # [serde (rename = "price")] pub price : i64 , # [serde (rename = "catId")] pub cat_id : i64 , # [serde (rename = "userId")] pub user_id : String } impl Schema for DragonsLobbyBuyCatParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"price\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"catId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"catId\",\"price\"]}") } } impl Agent for DragonsLobbyBuyCatParams { fn topic () -> & 'static str { "dragons_lobby_buyCat" } fn method () -> & 'static str { "lobby_buyCat" } fn agent () -> & 'static str { "dragons" } } impl < 'de > Deserialize < 'de > for DragonsLobbyBuyCatReturns { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (DragonsLobbyBuyCatReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyBuyCatReturns ; impl Schema for DragonsLobbyBuyCatReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyBuyCatReturns { fn topic () -> & 'static str { "dragons_lobby_buyCat" } fn method () -> & 'static str { "lobby_buyCat" } fn agent () -> & 'static str { "dragons" } }