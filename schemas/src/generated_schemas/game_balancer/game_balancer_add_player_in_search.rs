// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerAddPlayerInSearchParamsStatsParams {
    #[serde(rename = "attackRange")]
    pub attack_range: i16,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i16,
    #[serde(rename = "attackCritMultiple")]
    pub attack_crit_multiple: i16,
    #[serde(rename = "skillArmor")]
    pub skill_armor: i16,
    #[serde(rename = "ultRange")]
    pub ult_range: i16,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: i16,
    #[serde(rename = "dashReloadSpeed")]
    pub dash_reload_speed: i16,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i16,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: i16,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: i16,
    #[serde(rename = "ultChargeSpeed")]
    pub ult_charge_speed: i16,
    #[serde(rename = "dashRange")]
    pub dash_range: i16,
    #[serde(rename = "damage")]
    pub damage: i16,
    #[serde(rename = "attackCritChance")]
    pub attack_crit_chance: i16,
    #[serde(rename = "mainStat")]
    pub main_stat: i8,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i16,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i16,
    #[serde(rename = "dashSpeed")]
    pub dash_speed: i16,
    #[serde(rename = "skillPower")]
    pub skill_power: i16,
    #[serde(rename = "strength")]
    pub strength: i16,
    #[serde(rename = "attackArmor")]
    pub attack_armor: i16,
    #[serde(rename = "ultPower")]
    pub ult_power: i16,
    #[serde(rename = "intelligence")]
    pub intelligence: i16,
    #[serde(rename = "ultArmor")]
    pub ult_armor: i16,
    #[serde(rename = "health")]
    pub health: i16,
    #[serde(rename = "skillRange")]
    pub skill_range: i16,
    #[serde(rename = "skillReloadSpeed")]
    pub skill_reload_speed: i16,
    #[serde(rename = "agility")]
    pub agility: i16,
    #[serde(rename = "damageReturnPower")]
    pub damage_return_power: i16,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: i16,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerAddPlayerInSearchParams {
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "heroType")]
    pub hero_type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "stats")]
    pub stats: GameBalancerGameBalancerAddPlayerInSearchParamsStatsParams,
    #[serde(rename = "nickname")]
    pub nickname: String,
}
impl Schema for GameBalancerGameBalancerAddPlayerInSearchParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"region\":{\"type\":\"string\"},\"heroType\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"stats\":{\"type\":\"object\",\"properties\":{\"attackRange\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"attackCritMultiple\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"skillArmor\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"ultRange\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"dashReloadSpeed\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"attackSpeed\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"ultChargeSpeed\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"dashRange\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"damage\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"attackCritChance\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"mainStat\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"dashSpeed\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"attackArmor\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"ultArmor\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"health\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"skillRange\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"skillReloadSpeed\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"damageReturnPower\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"damage\",\"health\",\"mainStat\",\"healthRegenPercent\",\"skillArmor\",\"ultArmor\",\"moveSpeed\",\"attackDamage\",\"attackSpeed\",\"attackReloadSpeed\",\"attackArmor\",\"attackRange\",\"attackCritChance\",\"attackCritMultiple\",\"skillPower\",\"skillEffectPower\",\"skillReloadSpeed\",\"skillRange\",\"ultPower\",\"ultEffectPower\",\"ultChargeSpeed\",\"ultRange\",\"dashRange\",\"dashReloadSpeed\",\"dashSpeed\",\"vampirismPower\",\"damageReturnPower\"]},\"nickname\":{\"type\":\"string\"}},\"required\":[\"userId\",\"stats\",\"heroType\",\"nickname\",\"region\"]}") . unwrap ()
    }
}
impl Agent for GameBalancerGameBalancerAddPlayerInSearchParams {
    fn topic() -> &'static str {
        "gameBalancer_gameBalancer_addPlayerInSearch"
    }
    fn method() -> &'static str {
        "gameBalancer_addPlayerInSearch"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerAddPlayerInSearchReturns(pub bool);
impl Schema for GameBalancerGameBalancerAddPlayerInSearchReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for GameBalancerGameBalancerAddPlayerInSearchReturns {
    fn topic() -> &'static str {
        "gameBalancer_gameBalancer_addPlayerInSearch"
    }
    fn method() -> &'static str {
        "gameBalancer_addPlayerInSearch"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
}
