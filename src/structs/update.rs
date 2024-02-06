use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::common::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateData {
    pub primary: IndexMap<String, Value>,
    pub values: IndexMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateResponse {
    pub success: bool,
    pub error: Option<String>,
}
