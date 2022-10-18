// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for TaskTaskGetCurrentTasksParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(TaskTaskGetCurrentTasksParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TaskTaskGetCurrentTasksParams;
impl Schema for TaskTaskGetCurrentTasksParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetCurrentTasksReturnsTasksParamsParams {
    #[serde(rename = "QuestRewardId")]
    pub quest_reward_id: f64,
    #[serde(rename = "QuestId")]
    pub quest_id: f64,
    #[serde(rename = "RewardAmount")]
    pub reward_amount: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetCurrentTasksReturns {
    #[serde(rename = "EndDateMM")]
    pub end_date_mm: f64,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "Tasks")]
    pub tasks: Vec<TaskTaskGetCurrentTasksReturnsTasksParamsParams>,
}
impl Schema for TaskTaskGetCurrentTasksReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"EndDateMM\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"Tasks\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"QuestRewardId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"QuestId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"RewardAmount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"QuestId\",\"QuestRewardId\",\"RewardAmount\"]}}},\"required\":[\"status\",\"Tasks\",\"EndDateMM\"]}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
