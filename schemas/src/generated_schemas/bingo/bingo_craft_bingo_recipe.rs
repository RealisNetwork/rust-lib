// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct BingoBingoCraftBingoRecipeParams {
    #[serde(rename = "recipeId")]
    pub recipe_id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BingoBingoCraftBingoRecipeReturns {
    #[serde(rename = "jackpotRewardAmount")]
    pub jackpot_reward_amount: String,
    #[serde(rename = "bingoReceiptId")]
    pub bingo_receipt_id: i32,
    #[serde(rename = "hardRewardAmount")]
    pub hard_reward_amount: String,
}
