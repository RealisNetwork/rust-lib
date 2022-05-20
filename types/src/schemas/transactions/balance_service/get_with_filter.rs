use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetWithFilterSchema {
    #[serde(rename = "filterList")]
    pub filter_list: FilterList,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FilterList {
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "typeTransaction")]
    pub type_transaction: Option<String>,
    pub reason: Option<String>,
    pub creator: Option<String>,
    #[serde(rename = "firstDate")]
    pub first_date: Option<String>,
    #[serde(rename = "lastDate")]
    pub last_date: Option<String>,
    pub page: Option<usize>,
    #[serde(rename = "perPage")]
    pub per_page: Option<usize>,
}
