// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagesImageUploadImageParams {
    #[serde(rename = "extension")]
    pub extension: String,
    #[serde(rename = "binary")]
    pub binary: String,
}
impl Schema for ImagesImageUploadImageParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"extension\":{\"type\":\"string\"},\"binary\":{\"type\":\"string\"}},\"required\":[\"binary\",\"extension\"]}")
    }
}
impl Agent for ImagesImageUploadImageParams {
    fn topic() -> &'static str {
        "images_image_uploadImage"
    }
    fn method() -> &'static str {
        "image_uploadImage"
    }
    fn agent() -> &'static str {
        "images"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagesImageUploadImageReturns(pub String);
impl Schema for ImagesImageUploadImageReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for ImagesImageUploadImageReturns {
    fn topic() -> &'static str {
        "images_image_uploadImage"
    }
    fn method() -> &'static str {
        "image_uploadImage"
    }
    fn agent() -> &'static str {
        "images"
    }
}
