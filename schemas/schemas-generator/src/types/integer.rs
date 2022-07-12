use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "numberType")]
pub enum AdditionalAttribute {
    #[serde(alias = "byte")]
    Byte,
    #[serde(alias = "short")]
    Short,
    #[serde(alias = "int")]
    Int,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer {
    pub minimum: i64,
    pub maximum: i64,
    #[serde(rename = "additionalAttributes")]
    pub additional_attributes: AdditionalAttribute,
}
