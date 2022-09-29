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
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
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
pub struct BingoBingoGetBingoDataReturnsBingoItemsParamsParams {
    #[serde(rename = "ItemId")]
    pub item_id: i32,
    #[serde(rename = "Amount")]
    pub amount: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoDataReturnsBingoSeasonInfoParams {
    #[serde(rename = "SeasonStartDelayMs")]
    pub season_start_delay_ms: f64,
    #[serde(rename = "SeasonDurationMs")]
    pub season_duration_ms: f64,
    #[serde(rename = "SeasonId")]
    pub season_id: i32,
    #[serde(rename = "SeasonState")]
    pub season_state: i32,
    #[serde(rename = "LastUpdateTimeMs")]
    pub last_update_time_ms: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoGetBingoDataReturns {
    #[serde(rename = "BingoItems")]
    pub bingo_items: Vec<BingoBingoGetBingoDataReturnsBingoItemsParamsParams>,
    #[serde(rename = "BingoSeasonInfo")]
    pub bingo_season_info: BingoBingoGetBingoDataReturnsBingoSeasonInfoParams,
    #[serde(rename = "status")]
    pub status: i32,
}
impl Schema for BingoBingoGetBingoDataReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"BingoItems\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"ItemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"Amount\":{\"type\":\"string\"}},\"required\":[\"ItemId\",\"Amount\"]}},\"BingoSeasonInfo\":{\"type\":\"object\",\"properties\":{\"SeasonStartDelayMs\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"SeasonDurationMs\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"SeasonId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"SeasonState\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"LastUpdateTimeMs\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"SeasonId\",\"SeasonState\",\"LastUpdateTimeMs\",\"SeasonDurationMs\",\"SeasonStartDelayMs\"]},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"status\",\"BingoItems\",\"BingoSeasonInfo\"]}")
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
