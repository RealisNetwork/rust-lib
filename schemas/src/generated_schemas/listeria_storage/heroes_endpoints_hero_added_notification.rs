// This file are autogenerated on build, everything you write here will be replaced

# ! [allow (unknown_lints)] # ! [allow (clippy :: all)] use crate :: generated_schemas :: prelude :: * ; impl < 'de > Deserialize < 'de > for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams { fn deserialize < D > (_deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Ok (ListeriaStorageHeroesEndpointsHeroAddedNotificationParams) } } # [derive (Debug , Clone , Serialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationParams ; impl Schema for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams { fn schema () -> Value { serde_json :: json ! ("{}") } } impl Agent for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams { fn topic () -> & 'static str { "listeria-storage_heroesEndpoints_heroAddedNotification" } fn method () -> & 'static str { "heroesEndpoints_heroAddedNotification" } fn agent () -> & 'static str { "listeria-storage" } } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams { # [serde (rename = "itemUid")] pub item_uid : Option < i32 > , # [serde (rename = "equipment")] pub equipment : i8 } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsAttributesParams { # [serde (rename = "skillEffectPower")] pub skill_effect_power : String , # [serde (rename = "vampirismPower")] pub vampirism_power : String , # [serde (rename = "strength")] pub strength : String , # [serde (rename = "health")] pub health : String , # [serde (rename = "mainCharacteristic")] pub main_characteristic : i8 , # [serde (rename = "moveSpeed")] pub move_speed : String , # [serde (rename = "ultPower")] pub ult_power : String , # [serde (rename = "attackReloadSpeed")] pub attack_reload_speed : String , # [serde (rename = "agility")] pub agility : String , # [serde (rename = "healthRegenPercent")] pub health_regen_percent : String , # [serde (rename = "attackDamage")] pub attack_damage : String , # [serde (rename = "ultEffectPower")] pub ult_effect_power : String , # [serde (rename = "armor")] pub armor : String , # [serde (rename = "intelligence")] pub intelligence : String , # [serde (rename = "skillPower")] pub skill_power : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams { # [serde (rename = "ultPower")] pub ult_power : String , # [serde (rename = "mainCharacteristic")] pub main_characteristic : i8 , # [serde (rename = "strength")] pub strength : String , # [serde (rename = "armor")] pub armor : String , # [serde (rename = "agility")] pub agility : String , # [serde (rename = "attackDamage")] pub attack_damage : String , # [serde (rename = "skillEffectPower")] pub skill_effect_power : String , # [serde (rename = "moveSpeed")] pub move_speed : String , # [serde (rename = "ultEffectPower")] pub ult_effect_power : String , # [serde (rename = "vampirismPower")] pub vampirism_power : String , # [serde (rename = "skillPower")] pub skill_power : String , # [serde (rename = "intelligence")] pub intelligence : String , # [serde (rename = "attackReloadSpeed")] pub attack_reload_speed : String , # [serde (rename = "health")] pub health : String , # [serde (rename = "healthRegenPercent")] pub health_regen_percent : String } # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns { # [serde (rename = "level")] pub level : i16 , # [serde (rename = "toNextLevelScrolls")] pub to_next_level_scrolls : i32 , # [serde (rename = "slots")] pub slots : Vec < ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams > , # [serde (rename = "maxLevel")] pub max_level : i16 , # [serde (rename = "status")] pub status : i32 , # [serde (rename = "linkToExplorer")] pub link_to_explorer : String , # [serde (rename = "attributes")] pub attributes : ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsAttributesParams , # [serde (rename = "equipmentAttributes")] pub equipment_attributes : ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams , # [serde (rename = "active")] pub active : bool , # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "transactionHash")] pub transaction_hash : Option < String > , # [serde (rename = "toNextLevelExperience")] pub to_next_level_experience : i32 , # [serde (rename = "blockId")] pub block_id : Option < String > , # [serde (rename = "heroId")] pub hero_id : i8 , # [serde (rename = "isPending")] pub is_pending : bool } impl Schema for ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns { fn schema () -> Value { serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"equipment\"]}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"linkToExplorer\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"skillEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"moveSpeed\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"ultPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"strength\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"active\":{\"type\":\"boolean\"},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"transactionHash\":{\"type\":\"string\"},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"blockId\":{\"type\":\"string\"},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"isPending\":{\"type\":\"boolean\"}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}") } } impl Agent for ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns { fn topic () -> & 'static str { "listeria-storage_heroesEndpoints_heroAddedNotification" } fn method () -> & 'static str { "heroesEndpoints_heroAddedNotification" } fn agent () -> & 'static str { "listeria-storage" } }