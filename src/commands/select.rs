// TODO: return responses in json format

use std::{collections::HashMap, sync::Arc};

use scylla::serialize::value::SerializeCql;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

use crate::{
    state,
    structs::{self, Command},
    util::{parse_cql_value::parse_cql_value, queries::select_query},
};

impl SerializeCql for structs::Value {
    fn serialize<'b>(
        &self,
        typ: &scylla::frame::response::result::ColumnType,
        writer: scylla::serialize::CellWriter<'b>,
    ) -> Result<
        scylla::serialize::writers::WrittenCellProof<'b>,
        scylla::serialize::SerializationError,
    > {
        match self {
            &structs::Value::Str(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &structs::Value::Num(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &structs::Value::Int(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &structs::Value::Bool(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
            &structs::Value::Null => {
                scylla::serialize::value::SerializeCql::serialize(&None::<String>, typ, writer)
            }
            &structs::Value::Array(ref value) => {
                scylla::serialize::value::SerializeCql::serialize(value, typ, writer)
            }
        }
    }
}

pub async fn select(
    write: Arc<Mutex<OwnedWriteHalf>>,
    command: structs::CommandData,
    user: Arc<Mutex<state::ClientState>>,
    keyspace: Option<String>,
    table: Option<String>,
) {
    let mut write = write.lock().await;
    let uu = user.lock().await;

    if !uu.connected {
        let response = "Not connected to scylla";

        write.write_all(response.as_bytes()).await.unwrap();

        write.shutdown().await.unwrap();

        return;
    }

    match command {
        structs::CommandData::Select(select_data) => {
            let query = select_query(
                keyspace.to_owned().unwrap(),
                table.to_owned().unwrap(),
                select_data,
            );

            let session = uu.session.as_ref().unwrap().lock().await;

            match session.query(query.query, query.values).await {
                Ok(query_result) => {
                    let mut result = Vec::new();

                    let mut indexes = Vec::new();

                    let query_map = &query_result.col_specs.clone();

                    for value in query_map {
                        let (value_idx, _) =
                            query_result.get_column_spec(value.name.as_str()).unwrap();

                        indexes.push(value_idx);
                    }

                    for row in query_result.rows.unwrap() {
                        let mut row_vec: HashMap<String, structs::Value> = HashMap::new();

                        for index in indexes.clone() {
                            let column = row.columns[index].as_ref();
                            let name = query_map[index].name.as_str().to_string();

                            let value = parse_cql_value(column);

                            row_vec.insert(name, value);
                        }

                        result.push(row_vec);
                    }

                    let query_result = Command {
                        command: "select".to_string(),
                        keyspace: None,
                        table: None,
                        data: structs::CommandData::SelectResponse(structs::QueryResult {
                            result,
                            error: None,
                        }),
                    };

                    let response = serde_json::to_string(&query_result).unwrap();

                    write.write_all(response.as_bytes()).await.unwrap();
                }
                Err(error) => {
                    let query_result = Command {
                        command: "select".to_string(),
                        keyspace: None,
                        table: None,
                        data: structs::CommandData::SelectResponse(structs::QueryResult {
                            result: Vec::new(),
                            error: Some(error.to_string()),
                        }),
                    };

                    let response = serde_json::to_string(&query_result).unwrap();

                    write.write_all(response.as_bytes()).await.unwrap();
                }
            }
        }

        _ => {
            println!("Unknown command data: {:?}", command);
        }
    }
}
