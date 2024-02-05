use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::common::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteData {
    #[serde(rename = "where")]
    pub where_clause: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub error: Option<String>,
}
