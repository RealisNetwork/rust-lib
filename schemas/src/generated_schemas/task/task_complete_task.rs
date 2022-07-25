// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskCompleteTaskParams {
    #[serde(rename = "taskId")]
    pub task_id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for TaskTaskCompleteTaskParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"taskId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"taskId\"]}")
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskCompleteTaskReturns {
    #[serde(rename = "reward")]
    pub reward: i32,
    #[serde(rename = "taskId")]
    pub task_id: i32,
}
impl Schema for TaskTaskCompleteTaskReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"reward\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"taskId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"taskId\",\"reward\"]}")
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
}
