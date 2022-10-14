// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragocatsProductFactoryGeneratorGenerateParams { # [serde (rename = "personalType" , deserialize_with = "deserialize_to_string")] pub personal_type : String } impl Schema for DragocatsProductFactoryGeneratorGenerateParams { fn schema () -> Value { serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}") . unwrap () } } impl Agent for DragocatsProductFactoryGeneratorGenerateParams { fn topic () -> & 'static str { "dragocats-product-factory_generator_generate" } fn method () -> & 'static str { "generator_generate" } fn agent () -> & 'static str { "dragocats-product-factory" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragocatsProductFactoryGeneratorGenerateReturnsAttributesParams { # [serde (rename = "health" , deserialize_with = "deserialize_to_string")] pub health : String , # [serde (rename = "attackDamage" , deserialize_with = "deserialize_to_string")] pub attack_damage : String , # [serde (rename = "ultEffectPower" , deserialize_with = "deserialize_to_string")] pub ult_effect_power : String , # [serde (rename = "agility" , deserialize_with = "deserialize_to_string")] pub agility : String , # [serde (rename = "healthRegenPercent" , deserialize_with = "deserialize_to_string")] pub health_regen_percent : String , # [serde (rename = "strength" , deserialize_with = "deserialize_to_string")] pub strength : String , # [serde (rename = "skillEffectPower" , deserialize_with = "deserialize_to_string")] pub skill_effect_power : String , # [serde (rename = "intelligence" , deserialize_with = "deserialize_to_string")] pub intelligence : String , # [serde (rename = "moveSpeed" , deserialize_with = "deserialize_to_string")] pub move_speed : String , # [serde (rename = "ultPower" , deserialize_with = "deserialize_to_string")] pub ult_power : String , # [serde (rename = "skillPower" , deserialize_with = "deserialize_to_string")] pub skill_power : String , # [serde (rename = "armor" , deserialize_with = "deserialize_to_string")] pub armor : String , # [serde (rename = "mainCharacteristic" , deserialize_with = "deserialize_to_string")] pub main_characteristic : String , # [serde (rename = "attackReloadSpeed" , deserialize_with = "deserialize_to_string")] pub attack_reload_speed : String , # [serde (rename = "vampirismPower" , deserialize_with = "deserialize_to_string")] pub vampirism_power : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DragocatsProductFactoryGeneratorGenerateReturns { # [serde (rename = "name" , deserialize_with = "deserialize_to_string")] pub name : String , # [serde (rename = "mintId")] pub mint_id : f64 , # [serde (rename = "productType" , deserialize_with = "deserialize_to_string")] pub product_type : String , # [serde (rename = "attributes")] pub attributes : DragocatsProductFactoryGeneratorGenerateReturnsAttributesParams , # [serde (rename = "isNft")] pub is_nft : bool , # [serde (rename = "productId")] pub product_id : f64 , # [serde (rename = "personalType" , deserialize_with = "deserialize_to_string")] pub personal_type : String } impl Schema for DragocatsProductFactoryGeneratorGenerateReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"mintId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"productType\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"health\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"string\",\"pattern\":\"^(strength)|(agility)|(intelligence)$\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\",\"mainCharacteristic\"]},\"isNft\":{\"type\":\"boolean\"},\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\",\"productId\",\"productType\",\"name\",\"attributes\",\"mintId\",\"isNft\"]}") } } impl Agent for DragocatsProductFactoryGeneratorGenerateReturns { fn topic () -> & 'static str { "dragocats-product-factory_generator_generate" } fn method () -> & 'static str { "generator_generate" } fn agent () -> & 'static str { "dragocats-product-factory" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }