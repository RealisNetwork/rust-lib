// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyUserUpdateProfileImageParams {
    #[serde(rename = "image")]
    pub image: i8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyUserUpdateProfileImageReturns {
    #[serde(rename = "imageId")]
    pub image_id: i8,
}
