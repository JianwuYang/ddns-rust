use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct RecordItem {
    #[serde(alias = "RecordId")]
    pub record_id: i64,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Value")]
    pub value: String,
    #[serde(alias = "Remark")]
    pub remark: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(alias = "RecordList")]
    pub record_list: Vec<RecordItem>,
}
