// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbyGetUsersInappPurchasesParams { # [serde (rename = "startDate")] pub start_date : i64 , # [serde (rename = "endDate")] pub end_date : i64 , # [serde (rename = "userId")] pub user_id : String } impl Schema for DragonsLobbyGetUsersInappPurchasesParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"startDate\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"endDate\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"startDate\",\"endDate\"]}") } } impl Agent for DragonsLobbyGetUsersInappPurchasesParams { fn topic () -> & 'static str { "dragons_lobby_getUsersInappPurchases" } fn method () -> & 'static str { "lobby_getUsersInappPurchases" } fn agent () -> & 'static str { "dragons" } } impl < 'de > Deserialize < 'de > for DragonsLobbyGetUsersInappPurchasesReturns { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (DragonsLobbyGetUsersInappPurchasesReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyGetUsersInappPurchasesReturns ; impl Schema for DragonsLobbyGetUsersInappPurchasesReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyGetUsersInappPurchasesReturns { fn topic () -> & 'static str { "dragons_lobby_getUsersInappPurchases" } fn method () -> & 'static str { "lobby_getUsersInappPurchases" } fn agent () -> & 'static str { "dragons" } }