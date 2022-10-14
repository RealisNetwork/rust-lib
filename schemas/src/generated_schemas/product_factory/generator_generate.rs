// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryGeneratorGenerateParams { # [serde (rename = "personalType" , deserialize_with = "deserialize_to_string")] pub personal_type : String } impl Schema for ProductFactoryGeneratorGenerateParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}") . unwrap () } } impl Agent for ProductFactoryGeneratorGenerateParams { fn topic () -> & 'static str { "product-factory_generator_generate" } fn method () -> & 'static str { "generator_generate" } fn agent () -> & 'static str { "product-factory" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryGeneratorGenerateReturnsAttributesParams { # [serde (rename = "moveSpeed" , deserialize_with = "deserialize_to_string")] pub move_speed : String , # [serde (rename = "skillEffectPower" , deserialize_with = "deserialize_to_string")] pub skill_effect_power : String , # [serde (rename = "strength" , deserialize_with = "deserialize_to_string")] pub strength : String , # [serde (rename = "skillPower" , deserialize_with = "deserialize_to_string")] pub skill_power : String , # [serde (rename = "ultPower" , deserialize_with = "deserialize_to_string")] pub ult_power : String , # [serde (rename = "vampirismPower" , deserialize_with = "deserialize_to_string")] pub vampirism_power : String , # [serde (rename = "health" , deserialize_with = "deserialize_to_string")] pub health : String , # [serde (rename = "ultEffectPower" , deserialize_with = "deserialize_to_string")] pub ult_effect_power : String , # [serde (rename = "mainCharacteristic" , deserialize_with = "deserialize_to_string")] pub main_characteristic : String , # [serde (rename = "armor" , deserialize_with = "deserialize_to_string")] pub armor : String , # [serde (rename = "attackDamage" , deserialize_with = "deserialize_to_string")] pub attack_damage : String , # [serde (rename = "intelligence" , deserialize_with = "deserialize_to_string")] pub intelligence : String , # [serde (rename = "healthRegenPercent" , deserialize_with = "deserialize_to_string")] pub health_regen_percent : String , # [serde (rename = "attackReloadSpeed" , deserialize_with = "deserialize_to_string")] pub attack_reload_speed : String , # [serde (rename = "agility" , deserialize_with = "deserialize_to_string")] pub agility : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryGeneratorGenerateReturns { # [serde (rename = "productType" , deserialize_with = "deserialize_to_string")] pub product_type : String , # [serde (rename = "isNft")] pub is_nft : bool , # [serde (rename = "mintId")] pub mint_id : f64 , # [serde (rename = "productId")] pub product_id : f64 , # [serde (rename = "personalType" , deserialize_with = "deserialize_to_string")] pub personal_type : String , # [serde (rename = "name" , deserialize_with = "deserialize_to_string")] pub name : String , # [serde (rename = "attributes")] pub attributes : ProductFactoryGeneratorGenerateReturnsAttributesParams } impl Schema for ProductFactoryGeneratorGenerateReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"productType\":{\"type\":\"string\"},\"isNft\":{\"type\":\"boolean\"},\"mintId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"personalType\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"moveSpeed\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"string\",\"pattern\":\"^(strength)|(agility)|(intelligence)$\"},\"armor\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\",\"mainCharacteristic\"]}},\"required\":[\"personalType\",\"productId\",\"productType\",\"name\",\"attributes\",\"mintId\",\"isNft\"]}") } } impl Agent for ProductFactoryGeneratorGenerateReturns { fn topic () -> & 'static str { "product-factory_generator_generate" } fn method () -> & 'static str { "generator_generate" } fn agent () -> & 'static str { "product-factory" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }