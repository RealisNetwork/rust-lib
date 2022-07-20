// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyHeroesEndpointsGetEquippedItemsParams { # [serde (rename = "heroId")] pub hero_id : i32 } impl Schema for LobbyHeroesEndpointsGetEquippedItemsParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"heroId\"]}") } } impl Agent for LobbyHeroesEndpointsGetEquippedItemsParams { fn topic () -> & 'static str { "lobby_heroesEndpoints_getEquippedItems" } fn method () -> & 'static str { "heroesEndpoints_getEquippedItems" } fn agent () -> & 'static str { "lobby" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyHeroesEndpointsGetEquippedItemsReturnsParamsAttributesParams { # [serde (rename = "healthRegenPercent")] pub health_regen_percent : i64 , # [serde (rename = "attackReloadSpeed")] pub attack_reload_speed : i64 , # [serde (rename = "intelligence")] pub intelligence : i64 , # [serde (rename = "skillPower")] pub skill_power : i64 , # [serde (rename = "ultEffectPower")] pub ult_effect_power : i64 , # [serde (rename = "agility")] pub agility : i64 , # [serde (rename = "moveSpeed")] pub move_speed : i64 , # [serde (rename = "strength")] pub strength : i64 , # [serde (rename = "attackDamage")] pub attack_damage : i64 , # [serde (rename = "ultPower")] pub ult_power : i64 , # [serde (rename = "health")] pub health : i64 , # [serde (rename = "vampirismPower")] pub vampirism_power : i64 , # [serde (rename = "skillEffectPower")] pub skill_effect_power : i64 , # [serde (rename = "armor")] pub armor : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyHeroesEndpointsGetEquippedItemsReturnsParamsMultipliersParams { # [serde (rename = "health")] pub health : i64 , # [serde (rename = "skillPower")] pub skill_power : i64 , # [serde (rename = "skillEffectPower")] pub skill_effect_power : i64 , # [serde (rename = "attackReloadSpeed")] pub attack_reload_speed : i64 , # [serde (rename = "agility")] pub agility : i64 , # [serde (rename = "healthRegenPercent")] pub health_regen_percent : i64 , # [serde (rename = "ultEffectPower")] pub ult_effect_power : i64 , # [serde (rename = "attackDamage")] pub attack_damage : i64 , # [serde (rename = "vampirismPower")] pub vampirism_power : i64 , # [serde (rename = "strength")] pub strength : i64 , # [serde (rename = "moveSpeed")] pub move_speed : i64 , # [serde (rename = "armor")] pub armor : i64 , # [serde (rename = "ultPower")] pub ult_power : i64 , # [serde (rename = "intelligence")] pub intelligence : i64 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyHeroesEndpointsGetEquippedItemsReturnsParams { # [serde (rename = "itemId")] pub item_id : i32 , # [serde (rename = "attributes")] pub attributes : LobbyHeroesEndpointsGetEquippedItemsReturnsParamsAttributesParams , # [serde (rename = "level")] pub level : i32 , # [serde (rename = "userId")] pub user_id : String , # [serde (rename = "multipliers")] pub multipliers : LobbyHeroesEndpointsGetEquippedItemsReturnsParamsMultipliersParams , # [serde (rename = "type")] pub r#type : String , # [serde (rename = "slotId")] pub slot_id : i32 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LobbyHeroesEndpointsGetEquippedItemsReturns (Vec < LobbyHeroesEndpointsGetEquippedItemsReturnsParams >) ; impl Schema for LobbyHeroesEndpointsGetEquippedItemsReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attributes\":{\"type\":\"object\",\"properties\":{\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"multipliers\":{\"type\":\"object\",\"properties\":{\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"type\":{\"type\":\"string\"},\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"type\",\"slotId\",\"userId\",\"itemId\",\"level\",\"attributes\",\"multipliers\"]}}") } } impl Agent for LobbyHeroesEndpointsGetEquippedItemsReturns { fn topic () -> & 'static str { "lobby_heroesEndpoints_getEquippedItems" } fn method () -> & 'static str { "heroesEndpoints_getEquippedItems" } fn agent () -> & 'static str { "lobby" } }