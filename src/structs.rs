use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectData {
    #[serde(rename = "where")]
    pub where_clause: HashMap<String, Value>,
    pub columns: Vec<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    Str(String),
    Bool(bool),
    Num(f64),
    Int(i64),
    Null,
    Array(Vec<Value>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryResult {
    pub result: Vec<HashMap<String, Value>>,
    pub error: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct InsertData {
    pub columns: HashMap<String, Value>,
    #[serde(rename = "ifNotExists")]
    pub if_not_exists: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialsData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectData {
    #[serde(rename = "contactPoints")]
    pub contact_points: Vec<String>,
    #[serde(rename = "localDataCenter")]
    pub local_data_center: String,
    pub credentials: CredentialsData,
    pub keyspace: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CommandData {
    Select(SelectData),
    Insert(InsertData),
    Connect(ConnectData),
    SelectResponse(QueryResult),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub command: String,
    pub table: Option<String>,
    pub keyspace: Option<String>,
    pub data: CommandData,
}