// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyCraftBingoRecipeParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "recipeId")]
    pub recipe_id: f64,
}
impl Schema for DragonsLobbyCraftBingoRecipeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"recipeId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"recipeId\"]}")
    }
}
impl Agent for DragonsLobbyCraftBingoRecipeParams {
    fn topic() -> &'static str {
        "dragons_lobby_craftBingoRecipe"
    }
    fn method() -> &'static str {
        "lobby_craftBingoRecipe"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyCraftBingoRecipeReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyCraftBingoRecipeReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyCraftBingoRecipeReturns;
impl Schema for DragonsLobbyCraftBingoRecipeReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyCraftBingoRecipeReturns {
    fn topic() -> &'static str {
        "dragons_lobby_craftBingoRecipe"
    }
    fn method() -> &'static str {
        "lobby_craftBingoRecipe"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
