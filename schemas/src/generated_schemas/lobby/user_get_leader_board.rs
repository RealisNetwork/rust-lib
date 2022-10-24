// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyUserGetLeaderBoardParams {
    #[serde(rename = "page")]
    pub page: i16,
}
impl Schema for LobbyUserGetLeaderBoardParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}}},\"required\":[\"page\"]}") . unwrap ()
    }
}
impl Agent for LobbyUserGetLeaderBoardParams {
    fn topic() -> &'static str {
        "lobby_user_getLeaderBoard"
    }
    fn method() -> &'static str {
        "user_getLeaderBoard"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyUserGetLeaderBoardReturnsLeaderboardParamsParams {
    #[serde(rename = "place")]
    pub place: i32,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "rating")]
    pub rating: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyUserGetLeaderBoardReturns {
    #[serde(rename = "leaderboard")]
    pub leaderboard: Vec<LobbyUserGetLeaderBoardReturnsLeaderboardParamsParams>,
    #[serde(rename = "page")]
    pub page: i16,
    #[serde(rename = "pages")]
    pub pages: i16,
}
impl Schema for LobbyUserGetLeaderBoardReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"leaderboard\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"place\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"username\":{\"type\":\"string\"},\"rating\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"place\",\"username\",\"rating\"]}},\"page\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"pages\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}}},\"required\":[\"page\",\"pages\",\"leaderboard\"]}")
    }
}
impl Agent for LobbyUserGetLeaderBoardReturns {
    fn topic() -> &'static str {
        "lobby_user_getLeaderBoard"
    }
    fn method() -> &'static str {
        "user_getLeaderBoard"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
