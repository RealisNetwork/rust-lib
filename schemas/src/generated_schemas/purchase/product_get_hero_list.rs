// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for PurchaseProductGetHeroListParams { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde_json :: Value :: deserialize (deserializer) ? ; Ok (PurchaseProductGetHeroListParams) } } # [derive (Debug , Clone , Serialize)] pub struct PurchaseProductGetHeroListParams ; impl Schema for PurchaseProductGetHeroListParams { fn schema () -> Value { serde_json :: from_str ("{}") . unwrap () } } impl Agent for PurchaseProductGetHeroListParams { fn topic () -> & 'static str { "purchase_product_getHeroList" } fn method () -> & 'static str { "product_getHeroList" } fn agent () -> & 'static str { "purchase" } fn access_level () -> AccessLevel { AccessLevel :: Protected } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PurchaseProductGetHeroListReturnsParamsStatsParams { # [serde (rename = "skillEffectPower")] pub skill_effect_power : i32 , # [serde (rename = "attackReloadSpeed" , deserialize_with = "deserialize_to_string")] pub attack_reload_speed : String , # [serde (rename = "armor" , deserialize_with = "deserialize_to_string")] pub armor : String , # [serde (rename = "healthRegenPercent" , deserialize_with = "deserialize_to_string")] pub health_regen_percent : String , # [serde (rename = "ultPower")] pub ult_power : i32 , # [serde (rename = "agility")] pub agility : i32 , # [serde (rename = "skillPower")] pub skill_power : i32 , # [serde (rename = "attackDamage")] pub attack_damage : i32 , # [serde (rename = "vampirismPower" , deserialize_with = "deserialize_to_string")] pub vampirism_power : String , # [serde (rename = "moveSpeed" , deserialize_with = "deserialize_to_string")] pub move_speed : String , # [serde (rename = "mainCharacteristic")] pub main_characteristic : i8 , # [serde (rename = "health")] pub health : i32 , # [serde (rename = "strength")] pub strength : i32 , # [serde (rename = "intelligence")] pub intelligence : i32 , # [serde (rename = "ultEffectPower")] pub ult_effect_power : i32 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PurchaseProductGetHeroListReturnsParams { # [serde (rename = "currency" , deserialize_with = "deserialize_to_string")] pub currency : String , # [serde (rename = "productType" , deserialize_with = "deserialize_to_string")] pub product_type : String , # [serde (rename = "heroId")] pub hero_id : i8 , # [serde (rename = "stats")] pub stats : PurchaseProductGetHeroListReturnsParamsStatsParams , # [serde (rename = "price" , deserialize_with = "deserialize_to_string")] pub price : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PurchaseProductGetHeroListReturns (pub Vec < PurchaseProductGetHeroListReturnsParams >) ; impl Schema for PurchaseProductGetHeroListReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"productType\":{\"type\":\"string\"},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"stats\":{\"type\":\"object\",\"properties\":{\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attackReloadSpeed\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"vampirismPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"health\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"price\":{\"type\":\"string\"}},\"required\":[\"heroId\",\"productType\",\"currency\",\"price\",\"stats\"]}}") } } impl Agent for PurchaseProductGetHeroListReturns { fn topic () -> & 'static str { "purchase_product_getHeroList" } fn method () -> & 'static str { "product_getHeroList" } fn agent () -> & 'static str { "purchase" } fn access_level () -> AccessLevel { AccessLevel :: Protected } }