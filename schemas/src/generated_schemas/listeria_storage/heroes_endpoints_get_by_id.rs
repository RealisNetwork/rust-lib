// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetByIdParams {
    #[serde(rename = "heroId")]
    pub hero_id: i32,
}
impl Schema for ListeriaStorageHeroesEndpointsGetByIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"heroId\"]}") . unwrap ()
    }
}
impl Agent for ListeriaStorageHeroesEndpointsGetByIdParams {
    fn topic() -> &'static str {
        "listeria-storage_heroesEndpoints_getById"
    }
    fn method() -> &'static str {
        "heroesEndpoints_getById"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetByIdReturnsAttributesCoefficientsParams {
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: f64,
    #[serde(rename = "ultPower")]
    pub ult_power: f64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: f64,
    #[serde(rename = "armor")]
    pub armor: f64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: f64,
    #[serde(rename = "agility")]
    pub agility: f64,
    #[serde(rename = "skillPower")]
    pub skill_power: f64,
    #[serde(rename = "health")]
    pub health: f64,
    #[serde(rename = "strength")]
    pub strength: f64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: f64,
    #[serde(rename = "intelligence")]
    pub intelligence: f64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: f64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetByIdReturns {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "heroHashId")]
    pub hero_hash_id: String,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i16,
    #[serde(rename = "blockId")]
    pub block_id: String,
    #[serde(rename = "attributesCoefficients")]
    pub attributes_coefficients:
        ListeriaStorageHeroesEndpointsGetByIdReturnsAttributesCoefficientsParams,
}
impl Schema for ListeriaStorageHeroesEndpointsGetByIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"heroHashId\":{\"type\":\"string\"},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"blockId\":{\"type\":\"string\"},\"attributesCoefficients\":{\"type\":\"object\",\"properties\":{\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]}},\"required\":[\"id\",\"name\",\"blockId\",\"type\",\"userId\",\"heroHashId\",\"attributesCoefficients\",\"level\",\"experience\",\"toNextLevelExperience\",\"maxLevel\",\"toNextLevelScrolls\",\"status\"]}")
    }
}
impl Agent for ListeriaStorageHeroesEndpointsGetByIdReturns {
    fn topic() -> &'static str {
        "listeria-storage_heroesEndpoints_getById"
    }
    fn method() -> &'static str {
        "heroesEndpoints_getById"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
