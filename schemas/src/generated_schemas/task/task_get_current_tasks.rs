// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for TaskTaskGetCurrentTasksParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(TaskTaskGetCurrentTasksParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TaskTaskGetCurrentTasksParams;
impl Schema for TaskTaskGetCurrentTasksParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for TaskTaskGetCurrentTasksParams {
    fn topic() -> &'static str {
        "task_task_getCurrentTasks"
    }
    fn method() -> &'static str {
        "task_getCurrentTasks"
    }
    fn agent() -> &'static str {
        "task"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetCurrentTasksReturnsParams {
    #[serde(rename = "taskId")]
    pub task_id: i32,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "statusList")]
    pub status_list: Vec<String>,
    #[serde(rename = "rewardAmount")]
    pub reward_amount: i64,
    #[serde(rename = "rewardType")]
    pub reward_type: String,
    #[serde(rename = "taskTime")]
    pub task_time: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetCurrentTasksReturns(pub Vec<TaskTaskGetCurrentTasksReturnsParams>);
impl Schema for TaskTaskGetCurrentTasksReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"taskId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"personalType\":{\"type\":\"string\"},\"statusList\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"rewardAmount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"rewardType\":{\"type\":\"string\"},\"taskTime\":{\"type\":\"string\"}},\"required\":[\"taskId\",\"personalType\",\"statusList\",\"rewardType\",\"rewardAmount\",\"taskTime\"]}}")
    }
}
impl Agent for TaskTaskGetCurrentTasksReturns {
    fn topic() -> &'static str {
        "task_task_getCurrentTasks"
    }
    fn method() -> &'static str {
        "task_getCurrentTasks"
    }
    fn agent() -> &'static str {
        "task"
    }
}
