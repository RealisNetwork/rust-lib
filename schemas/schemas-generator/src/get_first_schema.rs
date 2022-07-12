// Add doc phrase that it's generated file {add json that we read}
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use crate::Agent;

// directory = agent_name
// file = method_name.rs
// mod.rs file with all imports for schemas

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthLoginParams {
    pub username: String,
    pub password: String,
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,
    #[serde(rename = "appId")]
    pub app_id: Option<i32>,
}
//
// impl Schema for AuthLoginParams {
//
// }
//
// impl Agent for AuthLoginParams {
//     fn topic() -> &'static str {
//         "auth_auth_login"
//     }
//
//     fn method() -> &'static str {
//         "auth_login"
//     }
//
//     fn agent() -> &'static str {
//         "auth"
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthLoginReturns {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_expires_in: i32,
    pub refresh_token: String,
    pub user_id: String,
}


// impl Schema for AuthLoginReturns {
//
// }
