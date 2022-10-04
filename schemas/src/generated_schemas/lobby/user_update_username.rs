// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyUserUpdateUsernameParams { # [serde (rename = "newUsername")] pub new_username : String } impl Schema for LobbyUserUpdateUsernameParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"newUsername\":{\"type\":\"string\"}},\"required\":[\"newUsername\"]}") . unwrap () } } impl Agent for LobbyUserUpdateUsernameParams { fn topic () -> & 'static str { "lobby_user_updateUsername" } fn method () -> & 'static str { "user_updateUsername" } fn agent () -> & 'static str { "lobby" } fn access_level () -> AccessLevel { } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyUserUpdateUsernameReturns { # [serde (rename = "newUsername")] pub new_username : String } impl Schema for LobbyUserUpdateUsernameReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"newUsername\":{\"type\":\"string\"}},\"required\":[\"newUsername\"]}") } } impl Agent for LobbyUserUpdateUsernameReturns { fn topic () -> & 'static str { "lobby_user_updateUsername" } fn method () -> & 'static str { "user_updateUsername" } fn agent () -> & 'static str { "lobby" } fn access_level () -> AccessLevel { } }