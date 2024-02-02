use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawData {
    pub query: String,
    pub values: Vec<super::common::Value>,
    pub limit: Option<i32>,
}
