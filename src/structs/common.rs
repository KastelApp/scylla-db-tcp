use indexmap::IndexMap;
use scylla::serialize::value::SerializeCql;
use serde::{Deserialize, Serialize};

use super::{
    connect::{ConnectData, ConnectResponse},
    insert::{InsertData, InsertResponse},
    select::SelectData,
    raw::RawData,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    Str(String),
    Bool(bool),
    Num(f64),
    Int(i64),
    Null,
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CommandData {
    Select(SelectData),
    Insert(InsertData),
    Connect(ConnectData),
    SelectResponse(QueryResult),
    InsertResponse(InsertResponse),
    Raw(RawData),
    ConnectResponse(ConnectResponse)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Command {
    pub hash: String, // ? The hash is for the command + table + keyspace + data
    pub command: String,
    pub table: Option<String>,
    pub keyspace: Option<String>,
    pub data: CommandData,
    pub length: usize, // ? The client sends the length of the data
    pub nonce: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryResult {
    pub result: Vec<IndexMap<String, Value>>,
    pub error: Option<String>,
}

impl SerializeCql for Value {
    fn serialize<'b>(
        &self,
        typ: &scylla::frame::response::result::ColumnType,
        writer: scylla::serialize::CellWriter<'b>,
    ) -> Result<
        scylla::serialize::writers::WrittenCellProof<'b>,
        scylla::serialize::SerializationError,
    > {
        match self {
            &Value::Str(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &Value::Num(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &Value::Int(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &Value::Bool(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &Value::Null => {
                scylla::serialize::value::SerializeCql::serialize(&None::<String>, typ, writer)
            }
            &Value::Array(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &Value::Map(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
        }
    }
}
