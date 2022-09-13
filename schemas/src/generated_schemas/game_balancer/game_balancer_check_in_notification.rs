// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for GameBalancerGameBalancerCheckInNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(GameBalancerGameBalancerCheckInNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct GameBalancerGameBalancerCheckInNotificationParams;
impl Schema for GameBalancerGameBalancerCheckInNotificationParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for GameBalancerGameBalancerCheckInNotificationParams {
    fn topic() -> &'static str {
        "gameBalancer_gameBalancer_checkInNotification"
    }
    fn method() -> &'static str {
        "gameBalancer_checkInNotification"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerCheckInNotificationReturnsPlayersParamsParamsAttributesParams {
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerCheckInNotificationReturnsPlayersParamsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "attributes")]
    pub attributes:
        GameBalancerGameBalancerCheckInNotificationReturnsPlayersParamsParamsAttributesParams,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerCheckInNotificationReturns {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "players")]
    pub players: Vec<GameBalancerGameBalancerCheckInNotificationReturnsPlayersParamsParams>,
    #[serde(rename = "roomId")]
    pub room_id: i32,
    #[serde(rename = "roomTcpPort")]
    pub room_tcp_port: i32,
    #[serde(rename = "roomHost")]
    pub room_host: String,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "teamId")]
    pub team_id: i8,
    #[serde(rename = "roomUdpPort")]
    pub room_udp_port: i32,
    #[serde(rename = "authToken")]
    pub auth_token: String,
}
impl Schema for GameBalancerGameBalancerCheckInNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"apiVersion\":{\"type\":\"string\"},\"players\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"nickname\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"skillPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"ultEffectPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]}},\"required\":[\"userId\",\"nickname\",\"heroId\",\"attributes\"]}},\"roomId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"roomTcpPort\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"roomHost\":{\"type\":\"string\"},\"success\":{\"type\":\"boolean\"},\"teamId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"roomUdpPort\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"authToken\":{\"type\":\"string\"}},\"required\":[\"success\",\"apiVersion\",\"authToken\",\"roomId\",\"roomHost\",\"roomTcpPort\",\"roomUdpPort\",\"teamId\",\"players\"]}")
    }
}
impl Agent for GameBalancerGameBalancerCheckInNotificationReturns {
    fn topic() -> &'static str {
        "gameBalancer_gameBalancer_checkInNotification"
    }
    fn method() -> &'static str {
        "gameBalancer_checkInNotification"
    }
    fn agent() -> &'static str {
        "gameBalancer"
    }
}
