// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragonsLobbyCraftBingoRecipeParams { # [serde (rename = "recipeId")] pub recipe_id : f64 , # [serde (rename = "userId")] pub user_id : String } impl Schema for DragonsLobbyCraftBingoRecipeParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"recipeId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"recipeId\"]}") . unwrap () } } impl Agent for DragonsLobbyCraftBingoRecipeParams { fn topic () -> & 'static str { "dragons_lobby_craftBingoRecipe" } fn method () -> & 'static str { "lobby_craftBingoRecipe" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { } } impl < 'de > Deserialize < 'de > for DragonsLobbyCraftBingoRecipeReturns { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (DragonsLobbyCraftBingoRecipeReturns) } } # [derive (Debug , Clone , Serialize)] pub struct DragonsLobbyCraftBingoRecipeReturns ; impl Schema for DragonsLobbyCraftBingoRecipeReturns { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for DragonsLobbyCraftBingoRecipeReturns { fn topic () -> & 'static str { "dragons_lobby_craftBingoRecipe" } fn method () -> & 'static str { "lobby_craftBingoRecipe" } fn agent () -> & 'static str { "dragons" } fn access_level () -> AccessLevel { } }