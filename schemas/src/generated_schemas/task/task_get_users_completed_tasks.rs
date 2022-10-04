// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetUsersCompletedTasksParams {}
impl Schema for TaskTaskGetUsersCompletedTasksParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for TaskTaskGetUsersCompletedTasksParams {
    fn topic() -> &'static str {
        "task_task_getUsersCompletedTasks"
    }
    fn method() -> &'static str {
        "task_getUsersCompletedTasks"
    }
    fn agent() -> &'static str {
        "task"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetUsersCompletedTasksReturns {
    #[serde(rename = "Tasks")]
    pub tasks: Vec<f64>,
    #[serde(rename = "status")]
    pub status: i32,
}
impl Schema for TaskTaskGetUsersCompletedTasksReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"Tasks\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"status\",\"Tasks\"]}")
    }
}
impl Agent for TaskTaskGetUsersCompletedTasksReturns {
    fn topic() -> &'static str {
        "task_task_getUsersCompletedTasks"
    }
    fn method() -> &'static str {
        "task_getUsersCompletedTasks"
    }
    fn agent() -> &'static str {
        "task"
    }
}
