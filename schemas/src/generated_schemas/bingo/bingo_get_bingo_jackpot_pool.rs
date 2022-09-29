// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BingoBingoGetBingoJackpotPoolParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(BingoBingoGetBingoJackpotPoolParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BingoBingoGetBingoJackpotPoolParams;
impl Schema for BingoBingoGetBingoJackpotPoolParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for BingoBingoGetBingoJackpotPoolParams {
    fn topic() -> &'static str {
        "bingo_bingo_getBingoJackpotPool"
    }
    fn method() -> &'static str {
        "bingo_getBingoJackpotPool"
    }
    fn agent() -> &'static str {
        "bingo"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoJackpotPoolReturns {
    #[serde(rename = "WinDate")]
    pub win_date: f64,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "Pool")]
    pub pool: String,
}
impl Schema for BingoBingoGetBingoJackpotPoolReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"WinDate\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"Pool\":{\"type\":\"string\"}},\"required\":[\"status\",\"Pool\",\"WinDate\"]}")
    }
}
impl Agent for BingoBingoGetBingoJackpotPoolReturns {
    fn topic() -> &'static str {
        "bingo_bingo_getBingoJackpotPool"
    }
    fn method() -> &'static str {
        "bingo_getBingoJackpotPool"
    }
    fn agent() -> &'static str {
        "bingo"
    }
}
