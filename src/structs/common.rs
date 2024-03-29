use std::{borrow::BorrowMut, collections::HashMap, fmt::Debug, hash::Hash, io::Stderr};

use indexmap::IndexMap;
use scylla::{frame::response::result::ColumnType, serialize::{row::SerializeRow, value::SerializeCql}, FromUserType, SerializeCql};
use serde::{Deserialize, Serialize};

use super::{
    connect::{ConnectData, ConnectResponse},
    insert::{InsertData, InsertResponse},
    raw::RawData,
    select::SelectData,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    Str(String),
    Bool(bool),
    Num(i32),
    Null,
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Date(String),
    // Object(HashMap<String, Value>),
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
    ConnectResponse(ConnectResponse),
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
        mut writer: scylla::serialize::CellWriter<'b>,
    ) -> Result<
        scylla::serialize::writers::WrittenCellProof<'b>,
        scylla::serialize::SerializationError,
    > {
        match self {
            &Value::Str(ref value) => {
                // scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
                match scylla::serialize::value::SerializeCql::serialize(value, typ, writer) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        println!("[Error] Failed to serialize Str: {:?}", err);
                        Err(err)
                    }
                }
            }
            &Value::Num(ref value) => {
                // scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
                match scylla::serialize::value::SerializeCql::serialize(value, typ, writer) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        println!("[Error] Failed to serialize Num: {:?}", err);
                        Err(err)
                    }
                }
            }
            // &Value::Int(ref value) => {
            //     scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            // }
            &Value::Bool(ref value) => {
                // scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
                match scylla::serialize::value::SerializeCql::serialize(value, typ, writer) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        println!("[Error] Failed to serialize Bool: {:?}", err);
                        Err(err)
                    }
                }
            }
            &Value::Null => {
                // scylla::serialize::value::SerializeCql::serialize(&None::<String>, typ, writer)
                match scylla::serialize::value::SerializeCql::serialize(
                    &None::<String>,
                    typ,
                    writer,
                ) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        println!("[Error] Failed to serialize Null: {:?}", err);
                        Err(err)
                    }
                }
            }
            &Value::Array(ref value) => {
                // scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
                match scylla::serialize::value::SerializeCql::serialize(value, typ, writer) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        println!("[Error] Failed to serialize Array: {:?}", err);
                        Err(err)
                    }
                }
            }
            &Value::Map(ref value) => {
                // scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
                match scylla::serialize::value::SerializeCql::serialize(value, typ, writer) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        println!("[Error] Failed to serialize Map: {:?}", err);
                        Err(err)
                    }
                }
            }
            &Value::Date(ref value) => {
                // scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
                match scylla::serialize::value::SerializeCql::serialize(value, typ, writer) {
                    Ok(value) => Ok(value),
                    Err(err) => {
                        println!("[Error] Failed to serialize Date: {:?}", err);
                        Err(err)
                    }
                }
            }
        //     &Value::Object(ref value) => {
        //         match writer.into_value_builder().append_bytes(value.to_string().as_bytes()).serialize(ctx, writer) {
        //             Ok(value) => Ok(value),
        //             Err(err) => {
        //                 println!("[Error] Failed to serialize Object: {:?}", err);
        //                 Err(scylla::serialize::SerializationError::new(std::io::Error::new(
        //                     std::io::ErrorKind::Other,
        //                     "Failed to serialize Object",
        //                 )))
        //             }
        //         }
                 
        //     }
        }
    }
}
