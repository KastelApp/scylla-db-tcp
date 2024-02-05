use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::common::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateData {
    #[serde(rename = "where")]
    pub where_clause: IndexMap<String, Value>,
    pub columns: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateResponse {
    pub success: bool,
    pub error: Option<String>,
}
