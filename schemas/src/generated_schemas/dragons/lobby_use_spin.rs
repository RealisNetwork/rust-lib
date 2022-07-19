// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyUseSpinParams {
    #[serde(rename = "spinTypeId")]
    pub spin_type_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyUseSpinParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"spinTypeId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"spinTypeId\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyUseSpinReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyUseSpinReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyUseSpinReturns;
impl Schema for DragonsLobbyUseSpinReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}