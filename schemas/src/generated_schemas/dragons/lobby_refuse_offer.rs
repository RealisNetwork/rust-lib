// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct DragonsLobbyRefuseOfferParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "offerKey")]
    pub offer_key: String,
}
pub type DragonsLobbyRefuseOfferReturns = ();
