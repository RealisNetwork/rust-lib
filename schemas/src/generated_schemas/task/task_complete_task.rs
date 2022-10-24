// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskCompleteTaskParams {
    #[serde(rename = "taskId")]
    pub task_id: f64,
}
impl Schema for TaskTaskCompleteTaskParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"taskId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"taskId\"]}") . unwrap ()
    }
}
impl Agent for TaskTaskCompleteTaskParams {
    fn topic() -> &'static str {
        "task_task_completeTask"
    }
    fn method() -> &'static str {
        "task_completeTask"
    }
    fn agent() -> &'static str {
        "task"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskCompleteTaskReturns {
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "TaskId")]
    pub task_id: f64,
}
impl Schema for TaskTaskCompleteTaskReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"TaskId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"status\",\"TaskId\"]}")
    }
}
impl Agent for TaskTaskCompleteTaskReturns {
    fn topic() -> &'static str {
        "task_task_completeTask"
    }
    fn method() -> &'static str {
        "task_completeTask"
    }
    fn agent() -> &'static str {
        "task"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
