// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetUsersCompletedTasksParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for TaskTaskGetUsersCompletedTasksParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
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
pub struct TaskTaskGetUsersCompletedTasksReturnsParamsTaskParams {
    #[serde(rename = "rewardType")]
    pub reward_type: String,
    #[serde(rename = "taskTime")]
    pub task_time: String,
    #[serde(rename = "statusList")]
    pub status_list: Vec<String>,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "taskId")]
    pub task_id: i32,
    #[serde(rename = "rewardAmount")]
    pub reward_amount: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetUsersCompletedTasksReturnsParams {
    #[serde(rename = "completeDate")]
    pub complete_date: String,
    #[serde(rename = "task")]
    pub task: TaskTaskGetUsersCompletedTasksReturnsParamsTaskParams,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTaskGetUsersCompletedTasksReturns(Vec<TaskTaskGetUsersCompletedTasksReturnsParams>);
impl Schema for TaskTaskGetUsersCompletedTasksReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"completeDate\":{\"type\":\"string\"},\"task\":{\"type\":\"object\",\"properties\":{\"rewardType\":{\"type\":\"string\"},\"taskTime\":{\"type\":\"string\"},\"statusList\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"personalType\":{\"type\":\"string\"},\"taskId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"rewardAmount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"taskId\",\"personalType\",\"statusList\",\"rewardType\",\"rewardAmount\",\"taskTime\"]},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"completeDate\",\"task\"]}}")
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