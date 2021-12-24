use std::fmt::Display;
use convert_case::{Case, Casing};
use serde_json::{json, Value};

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
