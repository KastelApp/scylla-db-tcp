use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::common::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SelectData {
    #[serde(rename = "where")]
    pub where_clause: IndexMap<String, Value>,
    pub columns: Vec<String>,
    pub limit: u32,
}
