use serde_json::{Value, json};
use serde::Serialize;

pub struct ResponseBuilder {}

impl ResponseBuilder {
    pub fn right<T, P>(request: T, value: P) -> Value
    where
        T: Serialize,
        P: Serialize
    {
        json!({
            "result": {
                "request": request,
                "response": {
                    "type": "Right",
                    "value": value
                }
            }
        })
    }

    pub fn left<T, P>(request: T, msg: P) -> Value
    where
        T: Serialize,
        P: ToString
    {
        json!({
            "result": {
                "request": request,
                "response": {
                    "type": "Left",
                    "value": {
                        "msg": msg.to_string()
                    }
                }
            }
        })
    }
}