// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for GameBalancerCoefficientEndpointsGetParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(GameBalancerCoefficientEndpointsGetParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct GameBalancerCoefficientEndpointsGetParams;
impl Schema for GameBalancerCoefficientEndpointsGetParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for GameBalancerCoefficientEndpointsGetParams {
    fn topic() -> &'static str {
        "gameBalancer_coefficientEndpoints_get"
    }
    fn method() -> &'static str {
        "coefficientEndpoints_get"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerCoefficientEndpointsGetReturns {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "strength")]
    pub strength: f64,
    #[serde(rename = "health")]
    pub health: f64,
    #[serde(rename = "armor")]
    pub armor: f64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: f64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: f64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: f64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: f64,
    #[serde(rename = "intelligence")]
    pub intelligence: f64,
    #[serde(rename = "agility")]
    pub agility: f64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f64,
    #[serde(rename = "skillPower")]
    pub skill_power: f64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: f64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: f64,
    #[serde(rename = "ultPower")]
    pub ult_power: f64,
}
impl Schema for GameBalancerCoefficientEndpointsGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"strength\",\"agility\",\"armor\",\"intelligence\",\"health\",\"healthRegenPercent\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]}")
    }
}
impl Agent for GameBalancerCoefficientEndpointsGetReturns {
    fn topic() -> &'static str {
        "gameBalancer_coefficientEndpoints_get"
    }
    fn method() -> &'static str {
        "coefficientEndpoints_get"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
}
