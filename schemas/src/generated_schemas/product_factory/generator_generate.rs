// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryGeneratorGenerateParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
impl Schema for ProductFactoryGeneratorGenerateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}") . unwrap ()
    }
}
impl Agent for ProductFactoryGeneratorGenerateParams {
    fn topic() -> &'static str {
        "product-factory_generator_generate"
    }
    fn method() -> &'static str {
        "generator_generate"
    }
    fn agent() -> &'static str {
        "product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryGeneratorGenerateReturnsAttributesParams {
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryGeneratorGenerateReturns {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "mintId")]
    pub mint_id: f64,
    #[serde(rename = "attributes")]
    pub attributes: ProductFactoryGeneratorGenerateReturnsAttributesParams,
    #[serde(rename = "productType")]
    pub product_type: String,
    #[serde(rename = "productId")]
    pub product_id: f64,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "isNft")]
    pub is_nft: bool,
}
impl Schema for ProductFactoryGeneratorGenerateReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"mintId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attributes\":{\"type\":\"object\",\"properties\":{\"skillEffectPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"string\",\"pattern\":\"^(strength)|(agility)|(intelligence)$\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\",\"mainCharacteristic\"]},\"productType\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"personalType\":{\"type\":\"string\"},\"isNft\":{\"type\":\"boolean\"}},\"required\":[\"personalType\",\"productId\",\"productType\",\"name\",\"attributes\",\"mintId\",\"isNft\"]}")
    }
}
impl Agent for ProductFactoryGeneratorGenerateReturns {
    fn topic() -> &'static str {
        "product-factory_generator_generate"
    }
    fn method() -> &'static str {
        "generator_generate"
    }
    fn agent() -> &'static str {
        "product-factory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
