// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryGeneratorGenerateParams { # [serde (rename = "personalType")] pub personal_type : String } impl Schema for ProductFactoryGeneratorGenerateParams { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}") } } impl Agent for ProductFactoryGeneratorGenerateParams { fn topic () -> & 'static str { "product-factory_generator_generate" } fn method () -> & 'static str { "generator_generate" } fn agent () -> & 'static str { "product-factory" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryGeneratorGenerateReturnsAttributesParams { # [serde (rename = "moveSpeed")] pub move_speed : String , # [serde (rename = "mainCharacteristic")] pub main_characteristic : String , # [serde (rename = "vampirismPower")] pub vampirism_power : String , # [serde (rename = "ultEffectPower")] pub ult_effect_power : String , # [serde (rename = "attackReloadSpeed")] pub attack_reload_speed : String , # [serde (rename = "healthRegenPercent")] pub health_regen_percent : String , # [serde (rename = "agility")] pub agility : String , # [serde (rename = "health")] pub health : String , # [serde (rename = "attackDamage")] pub attack_damage : String , # [serde (rename = "skillEffectPower")] pub skill_effect_power : String , # [serde (rename = "strength")] pub strength : String , # [serde (rename = "intelligence")] pub intelligence : String , # [serde (rename = "skillPower")] pub skill_power : String , # [serde (rename = "armor")] pub armor : String , # [serde (rename = "ultPower")] pub ult_power : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ProductFactoryGeneratorGenerateReturns { # [serde (rename = "isNft")] pub is_nft : bool , # [serde (rename = "personalType")] pub personal_type : String , # [serde (rename = "productType")] pub product_type : String , # [serde (rename = "productId")] pub product_id : i64 , # [serde (rename = "name")] pub name : String , # [serde (rename = "mintId")] pub mint_id : i64 , # [serde (rename = "attributes")] pub attributes : ProductFactoryGeneratorGenerateReturnsAttributesParams } impl Schema for ProductFactoryGeneratorGenerateReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isNft\":{\"type\":\"boolean\"},\"personalType\":{\"type\":\"string\"},\"productType\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"name\":{\"type\":\"string\"},\"mintId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attributes\":{\"type\":\"object\",\"properties\":{\"moveSpeed\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"string\",\"pattern\":\"^(strength)|(agility)|(intelligence)$\"},\"vampirismPower\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\",\"mainCharacteristic\"]}},\"required\":[\"personalType\",\"productId\",\"productType\",\"name\",\"attributes\",\"mintId\",\"isNft\"]}") } } impl Agent for ProductFactoryGeneratorGenerateReturns { fn topic () -> & 'static str { "product-factory_generator_generate" } fn method () -> & 'static str { "generator_generate" } fn agent () -> & 'static str { "product-factory" } }