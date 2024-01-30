use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::common::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertData {
    pub columns: IndexMap<String, Value>,
    #[serde(rename = "ifNotExists")]
    pub if_not_exists: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InsertResponse {
    pub success: bool,
    pub error: Option<String>,
}
