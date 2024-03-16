use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DDNSConfig {
    pub secret_id: String,
    pub secret_key: String,
    pub domain: String,
    pub sub_domain: Vec<String>,
    pub force_update: bool,
}