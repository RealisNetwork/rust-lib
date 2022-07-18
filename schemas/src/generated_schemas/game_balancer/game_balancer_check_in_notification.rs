// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
pub type GameBalancerGameBalancerCheckInNotificationParams = ();
#[derive(Debug, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerCheckInNotificationReturnsPlayersParamsParamsAttributesParams {
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerCheckInNotificationReturnsPlayersParamsParams {
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "attributes")]
    pub attributes:
        GameBalancerGameBalancerCheckInNotificationReturnsPlayersParamsParamsAttributesParams,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GameBalancerGameBalancerCheckInNotificationReturns {
    #[serde(rename = "roomHost")]
    pub room_host: String,
    #[serde(rename = "roomTcpPort")]
    pub room_tcp_port: i32,
    #[serde(rename = "roomUdpPort")]
    pub room_udp_port: i32,
    #[serde(rename = "teamId")]
    pub team_id: i8,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "roomId")]
    pub room_id: i32,
    #[serde(rename = "players")]
    pub players: Vec<GameBalancerGameBalancerCheckInNotificationReturnsPlayersParamsParams>,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "authToken")]
    pub auth_token: String,
}
