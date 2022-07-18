// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for DragonsLobbyGetBadTransactionsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetBadTransactionsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetBadTransactionsParams;
impl Schema for DragonsLobbyGetBadTransactionsParams {
    fn schema() -> Value {
        todo!()
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetBadTransactionsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetBadTransactionsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetBadTransactionsReturns;
