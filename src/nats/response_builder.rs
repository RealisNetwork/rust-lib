use serde::Serialize;
use serde_json::{json, Value};

pub struct ResponseBuilder {}

impl ResponseBuilder {

    #[must_use]
    pub fn build<T, P>(request: &T, response: &P) -> Value
        where
            T: Serialize,
            P: Serialize,
    {
        json!({
            "result": {
                "request": request,
                "response": response,
            }
        })
    }

    #[must_use]
    pub fn right<T, P>(request: T, value: P) -> Value
    where
        T: Serialize,
        P: Serialize,
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

    #[must_use]
    pub fn left<T, P>(request: T, msg: P) -> Value
    where
        T: Serialize,
        P: ToString,
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
