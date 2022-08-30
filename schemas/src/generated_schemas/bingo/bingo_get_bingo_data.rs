// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoDataParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for BingoBingoGetBingoDataParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for BingoBingoGetBingoDataParams {
    fn topic() -> &'static str {
        "bingo_bingo_getBingoData"
    }
    fn method() -> &'static str {
        "bingo_getBingoData"
    }
    fn agent() -> &'static str {
        "bingo"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoDataReturnsBingoSeasonInfoParams {
    #[serde(rename = "seasonState")]
    pub season_state: i32,
    #[serde(rename = "seasonId")]
    pub season_id: i32,
    #[serde(rename = "lastUpdateTimeMs")]
    pub last_update_time_ms: String,
    #[serde(rename = "seasonStartDelayMs")]
    pub season_start_delay_ms: String,
    #[serde(rename = "seasonDurationMs")]
    pub season_duration_ms: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoDataReturnsBingoItemsParamsParams {
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "amount")]
    pub amount: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoDataReturns {
    #[serde(rename = "bingoSeasonInfo")]
    pub bingo_season_info: BingoBingoGetBingoDataReturnsBingoSeasonInfoParams,
    #[serde(rename = "bingoItems")]
    pub bingo_items: Vec<BingoBingoGetBingoDataReturnsBingoItemsParamsParams>,
}
impl Schema for BingoBingoGetBingoDataReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"bingoSeasonInfo\":{\"type\":\"object\",\"properties\":{\"seasonState\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"seasonId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"lastUpdateTimeMs\":{\"type\":\"string\"},\"seasonStartDelayMs\":{\"type\":\"string\"},\"seasonDurationMs\":{\"type\":\"string\"}},\"required\":[\"seasonId\",\"seasonState\",\"lastUpdateTimeMs\",\"seasonDurationMs\",\"seasonStartDelayMs\"]},\"bingoItems\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"amount\":{\"type\":\"string\"}},\"required\":[\"itemId\",\"amount\"]}}},\"required\":[\"bingoItems\",\"bingoSeasonInfo\"]}")
    }
}
impl Agent for BingoBingoGetBingoDataReturns {
    fn topic() -> &'static str {
        "bingo_bingo_getBingoData"
    }
    fn method() -> &'static str {
        "bingo_getBingoData"
    }
    fn agent() -> &'static str {
        "bingo"
    }
}
