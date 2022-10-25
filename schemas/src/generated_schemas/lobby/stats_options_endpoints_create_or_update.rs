// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsScrollsByLevelsParamsParams
{
    #[serde(rename = "scrollsCount")]
    pub scrolls_count: f64,
    #[serde(rename = "level")]
    pub level: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsAttributesPerLevelParams
{
    #[serde(rename = "intelligence")]
    pub intelligence: f64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: f64,
    #[serde(rename = "skillPower")]
    pub skill_power: f64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f64,
    #[serde(rename = "strength")]
    pub strength: f64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: f64,
    #[serde(rename = "ultPower")]
    pub ult_power: f64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: f64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: f64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: f64,
    #[serde(rename = "health")]
    pub health: f64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: f64,
    #[serde(rename = "armor")]
    pub armor: f64,
    #[serde(rename = "agility")]
    pub agility: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsExperienceCoefficientsParamsParams
{
    #[serde(rename = "coefficient")]
    pub coefficient: f64,
    #[serde(rename = "level")]
    pub level: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParams { # [serde (rename = "baseScrollsCount")] pub base_scrolls_count : f64 , # [serde (rename = "scrollsByLevels")] pub scrolls_by_levels : Vec < LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsScrollsByLevelsParamsParams > , # [serde (rename = "strength")] pub strength : f64 , # [serde (rename = "intelligence")] pub intelligence : f64 , # [serde (rename = "attributesPerLevel")] pub attributes_per_level : LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsAttributesPerLevelParams , # [serde (rename = "agility")] pub agility : f64 , # [serde (rename = "baseExperience")] pub base_experience : f64 , # [serde (rename = "maxLevel")] pub max_level : f64 , # [serde (rename = "experienceCoefficients")] pub experience_coefficients : Vec < LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsExperienceCoefficientsParamsParams > , # [serde (rename = "experience")] pub experience : f64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsMultipliersRangesParams {
    #[serde(rename = "agility")]
    pub agility: Vec<f64>,
    #[serde(rename = "armor")]
    pub armor: Vec<f64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<f64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<f64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<f64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<f64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<f64>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<f64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<f64>,
    #[serde(rename = "health")]
    pub health: Vec<f64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<f64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<f64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<f64>,
    #[serde(rename = "strength")]
    pub strength: Vec<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsAttributesRangesParams {
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<f64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<f64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<f64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<f64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<f64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<f64>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<f64>,
    #[serde(rename = "armor")]
    pub armor: Vec<f64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<f64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<f64>,
    #[serde(rename = "strength")]
    pub strength: Vec<f64>,
    #[serde(rename = "agility")]
    pub agility: Vec<f64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<f64>,
    #[serde(rename = "health")]
    pub health: Vec<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParams {
    #[serde(rename = "levelUpOptions")]
    pub level_up_options:
        Option<LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParams>,
    #[serde(rename = "multipliersRanges")]
    pub multipliers_ranges:
        Option<LobbyStatsOptionsEndpointsCreateOrUpdateParamsMultipliersRangesParams>,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: Option<String>,
    #[serde(rename = "attributesRanges")]
    pub attributes_ranges:
        Option<LobbyStatsOptionsEndpointsCreateOrUpdateParamsAttributesRangesParams>,
    #[serde(rename = "personalType", deserialize_with = "deserialize_to_string")]
    pub personal_type: String,
}
impl Schema for LobbyStatsOptionsEndpointsCreateOrUpdateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"levelUpOptions\":{\"type\":\"object\",\"properties\":{\"baseScrollsCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"scrollsByLevels\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"scrollsCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"level\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"scrollsCount\",\"level\"]}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attributesPerLevel\":{\"type\":\"object\",\"properties\":{\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"baseExperience\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"experienceCoefficients\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"coefficient\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"level\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"coefficient\",\"level\"]}},\"experience\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"attributesPerLevel\",\"experience\",\"experienceCoefficients\",\"baseExperience\",\"baseScrollsCount\",\"scrollsByLevels\",\"maxLevel\"]},\"multipliersRanges\":{\"type\":\"object\",\"properties\":{\"agility\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"armor\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"intelligence\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"moveSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackReloadSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackDamage\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"health\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"healthRegenPercent\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"vampirismPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"strength\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"mainCharacteristic\":{\"type\":\"string\"},\"attributesRanges\":{\"type\":\"object\",\"properties\":{\"attackReloadSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"intelligence\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"healthRegenPercent\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackDamage\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"armor\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"vampirismPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"strength\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"agility\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"moveSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"health\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}") . unwrap ()
    }
}
impl Agent for LobbyStatsOptionsEndpointsCreateOrUpdateParams {
    fn topic() -> &'static str {
        "lobby_statsOptionsEndpoints_createOrUpdate"
    }
    fn method() -> &'static str {
        "statsOptionsEndpoints_createOrUpdate"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateReturns(pub bool);
impl Schema for LobbyStatsOptionsEndpointsCreateOrUpdateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for LobbyStatsOptionsEndpointsCreateOrUpdateReturns {
    fn topic() -> &'static str {
        "lobby_statsOptionsEndpoints_createOrUpdate"
    }
    fn method() -> &'static str {
        "statsOptionsEndpoints_createOrUpdate"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
