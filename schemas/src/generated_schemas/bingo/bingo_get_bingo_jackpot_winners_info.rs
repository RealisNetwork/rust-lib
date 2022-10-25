// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BingoBingoGetBingoJackpotWinnersInfoParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(BingoBingoGetBingoJackpotWinnersInfoParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BingoBingoGetBingoJackpotWinnersInfoParams;
impl Schema for BingoBingoGetBingoJackpotWinnersInfoParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for BingoBingoGetBingoJackpotWinnersInfoParams {
    fn topic() -> &'static str {
        "bingo_bingo_getBingoJackpotWinnersInfo"
    }
    fn method() -> &'static str {
        "bingo_getBingoJackpotWinnersInfo"
    }
    fn agent() -> &'static str {
        "bingo"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoJackpotWinnersInfoReturnsWinnersParamsParams {
    #[serde(rename = "StatusId")]
    pub status_id: f64,
    #[serde(rename = "Windate")]
    pub windate: f64,
    #[serde(rename = "Nick", deserialize_with = "deserialize_to_string")]
    pub nick: String,
    #[serde(rename = "Reward")]
    pub reward: f64,
    #[serde(rename = "UserId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoJackpotWinnersInfoReturns {
    #[serde(rename = "Winners")]
    pub winners: Vec<BingoBingoGetBingoJackpotWinnersInfoReturnsWinnersParamsParams>,
    #[serde(rename = "status")]
    pub status: i32,
}
impl Schema for BingoBingoGetBingoJackpotWinnersInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"Winners\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"StatusId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"Windate\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"Nick\":{\"type\":\"string\"},\"Reward\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"UserId\":{\"type\":\"string\"}},\"required\":[\"Nick\",\"UserId\",\"Reward\",\"StatusId\",\"Windate\"]}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"status\",\"Winners\"]}")
    }
}
impl Agent for BingoBingoGetBingoJackpotWinnersInfoReturns {
    fn topic() -> &'static str {
        "bingo_bingo_getBingoJackpotWinnersInfo"
    }
    fn method() -> &'static str {
        "bingo_getBingoJackpotWinnersInfo"
    }
    fn agent() -> &'static str {
        "bingo"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
