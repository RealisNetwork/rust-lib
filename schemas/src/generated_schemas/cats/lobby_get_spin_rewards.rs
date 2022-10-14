// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CatsLobbyGetSpinRewardsParams { # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String } impl Schema for CatsLobbyGetSpinRewardsParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap () } } impl Agent for CatsLobbyGetSpinRewardsParams { fn topic () -> & 'static str { "cats_lobby_getSpinRewards" } fn method () -> & 'static str { "lobby_getSpinRewards" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Public } } impl < 'de > Deserialize < 'de > for CatsLobbyGetSpinRewardsReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (CatsLobbyGetSpinRewardsReturns) } } # [derive (Debug , Clone , Serialize)] pub struct CatsLobbyGetSpinRewardsReturns ; impl Schema for CatsLobbyGetSpinRewardsReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for CatsLobbyGetSpinRewardsReturns { fn topic () -> & 'static str { "cats_lobby_getSpinRewards" } fn method () -> & 'static str { "lobby_getSpinRewards" } fn agent () -> & 'static str { "cats" } fn access_level () -> AccessLevel { AccessLevel :: Public } }