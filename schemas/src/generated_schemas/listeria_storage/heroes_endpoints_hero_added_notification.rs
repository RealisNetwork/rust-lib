// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (ListeriaStorageHeroesEndpointsHeroAddedNotificationParams) } } # [derive (Debug , Clone , Serialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationParams ; impl Schema for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams { fn topic () -> & 'static str { "listeria-storage_heroesEndpoints_heroAddedNotification" } fn method () -> & 'static str { "heroesEndpoints_heroAddedNotification" } fn agent () -> & 'static str { "listeria-storage" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsAttributesParams { # [serde (rename = "skillEffectPower")] pub skill_effect_power : String , # [serde (rename = "mainCharacteristic")] pub main_characteristic : i8 , # [serde (rename = "intelligence")] pub intelligence : String , # [serde (rename = "ultEffectPower")] pub ult_effect_power : String , # [serde (rename = "armor")] pub armor : String , # [serde (rename = "ultPower")] pub ult_power : String , # [serde (rename = "attackReloadSpeed")] pub attack_reload_speed : String , # [serde (rename = "skillPower")] pub skill_power : String , # [serde (rename = "health")] pub health : String , # [serde (rename = "moveSpeed")] pub move_speed : String , # [serde (rename = "vampirismPower")] pub vampirism_power : String , # [serde (rename = "agility")] pub agility : String , # [serde (rename = "healthRegenPercent")] pub health_regen_percent : String , # [serde (rename = "strength")] pub strength : String , # [serde (rename = "attackDamage")] pub attack_damage : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams { # [serde (rename = "equipment")] pub equipment : i8 , # [serde (rename = "itemUid")] pub item_uid : Option < i32 > } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams { # [serde (rename = "agility")] pub agility : String , # [serde (rename = "ultPower")] pub ult_power : String , # [serde (rename = "attackDamage")] pub attack_damage : String , # [serde (rename = "health")] pub health : String , # [serde (rename = "ultEffectPower")] pub ult_effect_power : String , # [serde (rename = "vampirismPower")] pub vampirism_power : String , # [serde (rename = "strength")] pub strength : String , # [serde (rename = "intelligence")] pub intelligence : String , # [serde (rename = "healthRegenPercent")] pub health_regen_percent : String , # [serde (rename = "attackReloadSpeed")] pub attack_reload_speed : String , # [serde (rename = "skillEffectPower")] pub skill_effect_power : String , # [serde (rename = "armor")] pub armor : String , # [serde (rename = "moveSpeed")] pub move_speed : String , # [serde (rename = "skillPower")] pub skill_power : String , # [serde (rename = "mainCharacteristic")] pub main_characteristic : i8 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns { # [serde (rename = "attributes")] pub attributes : ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsAttributesParams , # [serde (rename = "level")] pub level : i16 , # [serde (rename = "slots")] pub slots : Vec < ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams > , # [serde (rename = "blockId")] pub block_id : Option < String > , # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "equipmentAttributes")] pub equipment_attributes : ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams , # [serde (rename = "toNextLevelExperience")] pub to_next_level_experience : i32 , # [serde (rename = "status")] pub status : i32 , # [serde (rename = "toNextLevelScrolls")] pub to_next_level_scrolls : i32 , # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "linkToExplorer")] pub link_to_explorer : String , # [serde (rename = "maxLevel")] pub max_level : i16 , # [serde (rename = "heroId")] pub hero_id : i8 , # [serde (rename = "isPending")] pub is_pending : bool , # [serde (rename = "active")] pub active : bool , # [serde (rename = "transactionHash")] pub transaction_hash : Option < String > } impl Schema for ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"attributes\":{\"type\":\"object\",\"properties\":{\"skillEffectPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"intelligence\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"equipment\"]}},\"blockId\":{\"type\":\"string\"},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"agility\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"linkToExplorer\":{\"type\":\"string\"},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"isPending\":{\"type\":\"boolean\"},\"active\":{\"type\":\"boolean\"},\"transactionHash\":{\"type\":\"string\"}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}") } } impl Agent for ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns { fn topic () -> & 'static str { "listeria-storage_heroesEndpoints_heroAddedNotification" } fn method () -> & 'static str { "heroesEndpoints_heroAddedNotification" } fn agent () -> & 'static str { "listeria-storage" } }