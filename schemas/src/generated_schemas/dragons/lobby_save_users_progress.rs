// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbySaveUsersProgressParams { # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String , # [serde (rename = "dataObject" , deserialize_with = "deserialize_to_string")] pub data_object : String } impl Schema for DragonsLobbySaveUsersProgressParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"dataObject\":{\"type\":\"string\"}},\"required\":[\"userId\",\"dataObject\"]}") . unwrap () } } impl Agent for DragonsLobbySaveUsersProgressParams { fn topic () -> & 'static str { "dragons_lobby_saveUsersProgress" } fn method () -> & 'static str { "lobby_saveUsersProgress" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { AccessLevel :: Public } } impl < 'de > Deserialize < 'de > for DragonsLobbySaveUsersProgressReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragonsLobbySaveUsersProgressReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbySaveUsersProgressReturns ; impl Schema for DragonsLobbySaveUsersProgressReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbySaveUsersProgressReturns { fn topic () -> & 'static str { "dragons_lobby_saveUsersProgress" } fn method () -> & 'static str { "lobby_saveUsersProgress" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { AccessLevel :: Public } }