// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoCraftBingoRecipeParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "recipeId")]
    pub recipe_id: i32,
}
impl Schema for BingoBingoCraftBingoRecipeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"recipeId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"userId\",\"recipeId\"]}")
    }
}
impl Agent for BingoBingoCraftBingoRecipeParams {
    fn topic() -> &'static str {
        "bingo_bingo_craftBingoRecipe"
    }
    fn method() -> &'static str {
        "bingo_craftBingoRecipe"
    }
    fn agent() -> &'static str {
        "bingo"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoBingoCraftBingoRecipeReturns {
    #[serde(rename = "BingoReceiptId")]
    pub bingo_receipt_id: i32,
    #[serde(rename = "JackpotRewardAmount")]
    pub jackpot_reward_amount: String,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "HardRewardAmount")]
    pub hard_reward_amount: String,
}
impl Schema for BingoBingoCraftBingoRecipeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"BingoReceiptId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"JackpotRewardAmount\":{\"type\":\"string\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"HardRewardAmount\":{\"type\":\"string\"}},\"required\":[\"status\",\"BingoReceiptId\",\"HardRewardAmount\",\"JackpotRewardAmount\"]}")
    }
}
impl Agent for BingoBingoCraftBingoRecipeReturns {
    fn topic() -> &'static str {
        "bingo_bingo_craftBingoRecipe"
    }
    fn method() -> &'static str {
        "bingo_craftBingoRecipe"
    }
    fn agent() -> &'static str {
        "bingo"
    }
}
