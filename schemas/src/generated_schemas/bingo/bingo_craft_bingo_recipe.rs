// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BingoBingoCraftBingoRecipeParams { # [serde (rename = "recipeId")] pub recipe_id : i32 , # [serde (rename = "userId" , deserialize_with = "deserialize_to_string")] pub user_id : String } impl Schema for BingoBingoCraftBingoRecipeParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"recipeId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"recipeId\"]}") . unwrap () } } impl Agent for BingoBingoCraftBingoRecipeParams { fn topic () -> & 'static str { "bingo_bingo_craftBingoRecipe" } fn method () -> & 'static str { "bingo_craftBingoRecipe" } fn agent () -> & 'static str { "bingo" } fn access_level () -> AccessLevel { AccessLevel :: Internal } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct BingoBingoCraftBingoRecipeReturns { # [serde (rename = "HardRewardAmount" , deserialize_with = "deserialize_to_string")] pub hard_reward_amount : String , # [serde (rename = "JackpotRewardAmount" , deserialize_with = "deserialize_to_string")] pub jackpot_reward_amount : String , # [serde (rename = "status")] pub status : i32 , # [serde (rename = "BingoReceiptId")] pub bingo_receipt_id : i32 } impl Schema for BingoBingoCraftBingoRecipeReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"HardRewardAmount\":{\"type\":\"string\"},\"JackpotRewardAmount\":{\"type\":\"string\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"BingoReceiptId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"status\",\"BingoReceiptId\",\"HardRewardAmount\",\"JackpotRewardAmount\"]}") } } impl Agent for BingoBingoCraftBingoRecipeReturns { fn topic () -> & 'static str { "bingo_bingo_craftBingoRecipe" } fn method () -> & 'static str { "bingo_craftBingoRecipe" } fn agent () -> & 'static str { "bingo" } fn access_level () -> AccessLevel { AccessLevel :: Internal } }