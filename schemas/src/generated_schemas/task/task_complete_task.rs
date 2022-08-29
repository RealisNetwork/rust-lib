// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskCompleteTaskParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "taskId")]
    pub task_id: f64,
}
impl Schema for TaskTaskCompleteTaskParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"taskId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"taskId\"]}")
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
    #[serde(rename = "TaskId")]
    pub task_id: f64,
}
impl Schema for TaskTaskCompleteTaskReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"TaskId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"TaskId\"]}")
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
