// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CatsLobbyGetUsersCardsParams { # [serde (rename = "userId")] pub user_id : String } impl Schema for CatsLobbyGetUsersCardsParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") } } impl Agent for CatsLobbyGetUsersCardsParams { fn topic () -> & 'static str { "cats_lobby_getUsersCards" } fn method () -> & 'static str { "lobby_getUsersCards" } fn agent () -> & 'static str { "cats" } } impl < 'de > Deserialize < 'de > for CatsLobbyGetUsersCardsReturns { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (CatsLobbyGetUsersCardsReturns) } } # [derive (Debug , Clone , Serialize)] pub struct CatsLobbyGetUsersCardsReturns ; impl Schema for CatsLobbyGetUsersCardsReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for CatsLobbyGetUsersCardsReturns { fn topic () -> & 'static str { "cats_lobby_getUsersCards" } fn method () -> & 'static str { "lobby_getUsersCards" } fn agent () -> & 'static str { "cats" } }