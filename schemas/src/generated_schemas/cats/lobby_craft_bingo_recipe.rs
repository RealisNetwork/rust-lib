// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyCraftBingoRecipeParams {
    #[serde(rename = "recipeId")]
    pub recipe_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyCraftBingoRecipeParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"recipeId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"recipeId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyCraftBingoRecipeParams {
    fn topic() -> &'static str {
        "cats_lobby_craftBingoRecipe"
    }
    fn method() -> &'static str {
        "lobby_craftBingoRecipe"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for CatsLobbyCraftBingoRecipeReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyCraftBingoRecipeReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyCraftBingoRecipeReturns;
impl Schema for CatsLobbyCraftBingoRecipeReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyCraftBingoRecipeReturns {
    fn topic() -> &'static str {
        "cats_lobby_craftBingoRecipe"
    }
    fn method() -> &'static str {
        "lobby_craftBingoRecipe"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
