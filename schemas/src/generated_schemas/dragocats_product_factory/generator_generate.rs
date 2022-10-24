// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsProductFactoryGeneratorGenerateParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
impl Schema for DragocatsProductFactoryGeneratorGenerateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}") . unwrap ()
    }
}
impl Agent for DragocatsProductFactoryGeneratorGenerateParams {
    fn topic() -> &'static str {
        "dragocats-product-factory_generator_generate"
    }
    fn method() -> &'static str {
        "generator_generate"
    }
    fn agent() -> &'static str {
        "dragocats-product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsProductFactoryGeneratorGenerateReturnsAttributesParams {
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsProductFactoryGeneratorGenerateReturns {
    #[serde(rename = "productType")]
    pub product_type: String,
    #[serde(rename = "isNft")]
    pub is_nft: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "attributes")]
    pub attributes: DragocatsProductFactoryGeneratorGenerateReturnsAttributesParams,
    #[serde(rename = "productId")]
    pub product_id: f64,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "mintId")]
    pub mint_id: f64,
}
impl Schema for DragocatsProductFactoryGeneratorGenerateReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"productType\":{\"type\":\"string\"},\"isNft\":{\"type\":\"boolean\"},\"name\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"armor\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"string\",\"pattern\":\"^(strength)|(agility)|(intelligence)$\"},\"skillPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\",\"mainCharacteristic\"]},\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"personalType\":{\"type\":\"string\"},\"mintId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"personalType\",\"productId\",\"productType\",\"name\",\"attributes\",\"mintId\",\"isNft\"]}")
    }
}
impl Agent for DragocatsProductFactoryGeneratorGenerateReturns {
    fn topic() -> &'static str {
        "dragocats-product-factory_generator_generate"
    }
    fn method() -> &'static str {
        "generator_generate"
    }
    fn agent() -> &'static str {
        "dragocats-product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
