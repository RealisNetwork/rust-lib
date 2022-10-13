// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyUserGetUserDataParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbyUserGetUserDataParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyUserGetUserDataParams;
impl Schema for LobbyUserGetUserDataParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for LobbyUserGetUserDataParams {
    fn topic() -> &'static str {
        "lobby_user_getUserData"
    }
    fn method() -> &'static str {
        "user_getUserData"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyUserGetUserDataReturns {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "image")]
    pub image: i8,
    #[serde(rename = "increaseBy")]
    pub increase_by: i8,
    #[serde(rename = "rating")]
    pub rating: i32,
    #[serde(rename = "placeInLeaderBoard")]
    pub place_in_leader_board: i32,
    #[serde(rename = "draw")]
    pub draw: i8,
    #[serde(rename = "tokensIncreaseBy")]
    pub tokens_increase_by: String,
    #[serde(rename = "decreaseBy")]
    pub decrease_by: i8,
    #[serde(rename = "email")]
    pub email: String,
}
impl Schema for LobbyUserGetUserDataReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"username\":{\"type\":\"string\"},\"image\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"increaseBy\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"rating\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"placeInLeaderBoard\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"draw\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"tokensIncreaseBy\":{\"type\":\"string\"},\"decreaseBy\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"email\":{\"type\":\"string\"}},\"required\":[\"username\",\"rating\",\"increaseBy\",\"decreaseBy\",\"draw\",\"placeInLeaderBoard\",\"image\",\"email\",\"tokensIncreaseBy\"]}")
    }
}
impl Agent for LobbyUserGetUserDataReturns {
    fn topic() -> &'static str {
        "lobby_user_getUserData"
    }
    fn method() -> &'static str {
        "user_getUserData"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
