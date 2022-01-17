use convert_case::{Case, Casing};
use serde_json::{json, Value};
use std::fmt::Display;

pub trait ToJson: Display {
    fn as_string(&self) -> String {
        format!("{}", self).to_case(Case::Camel)
    }

    fn to_json(&self) -> Value {
        json!({
            "type": "Left",
            "value": {
                "type": self.as_string()
            }
        })
    }
    fn to_json_with_msg(&self, msg: &str) -> Value {
        json!({
            "type": "Left",
            "value": {
                "msg": msg,
                "type": self.as_string()
            }
        })
    }
}

impl ToJson for &i32 {}

impl ToJson for &String {}
