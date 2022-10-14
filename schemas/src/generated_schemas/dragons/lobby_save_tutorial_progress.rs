// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbySaveTutorialProgressParams { # [serde (rename = "dataObject" , deserialize_with = "deserialize_to_string")] pub data_object : String , # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String } impl Schema for DragonsLobbySaveTutorialProgressParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"dataObject\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"dataObject\"]}") . unwrap () } } impl Agent for DragonsLobbySaveTutorialProgressParams { fn topic () -> & 'static str { "dragons_lobby_saveTutorialProgress" } fn method () -> & 'static str { "lobby_saveTutorialProgress" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { AccessLevel :: Public } } impl < 'de > Deserialize < 'de > for DragonsLobbySaveTutorialProgressReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragonsLobbySaveTutorialProgressReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbySaveTutorialProgressReturns ; impl Schema for DragonsLobbySaveTutorialProgressReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbySaveTutorialProgressReturns { fn topic () -> & 'static str { "dragons_lobby_saveTutorialProgress" } fn method () -> & 'static str { "lobby_saveTutorialProgress" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { AccessLevel :: Public } }