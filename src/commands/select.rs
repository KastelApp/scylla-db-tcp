use futures_util::{stream::SplitSink, SinkExt};
use indexmap::IndexMap;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    calculate_hash::calculate_hash,
    state,
    structs::common::{Command, CommandData, QueryResult, Value},
    util::{parse_cql_value::parse_cql_value, queries::select_query},
    LOGGING,
};

pub async fn select(
    write: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    command: &CommandData,
    user: Arc<Mutex<state::ClientState>>,
    keyspace: &Option<String>,
    table: &Option<String>,
    raw_command: &Command,
) {
    let user = user.lock().await;

    if !user.connected {
        let error = Command {
            command: "error".to_string(),
            data: CommandData::SelectResponse(QueryResult {
                error: Some("Not connected to Scylla".to_string()),
                result: Vec::new(),
            }),
            keyspace: None,
            table: None,
            hash: "".to_string(),
            length: "".len(),
            nonce: None,
        };

        let response = serde_json::to_string(&error).unwrap();

        let mut write = write.lock().await;

        match write.send(Message::Text(response)).await {
            // ? we don't care about if it succeeds or not
            _ => {}
        }

        match write.close().await {
            _ => {}
        }

        return;
    }

    match command {
        CommandData::Select(select_data) => {
            let table = table.as_ref().unwrap();
            let keyspace = keyspace.as_ref();
            let user_keyspace = &user.keyspace;

            let query = select_query(
                &keyspace.to_owned().unwrap_or(user_keyspace),
                table,
                select_data,
            );

            let session = user.session.as_ref().unwrap().lock().await;

            match session.query(query.query, query.values).await {
                Ok(query_result) => {
                    let mut result = Vec::new();

                    let mut indexes = Vec::new();

                    let query_map = &query_result.col_specs;

                    for value in query_map {
                        let (value_idx, _) =
                            query_result.get_column_spec(value.name.as_str()).unwrap();

                        indexes.push(value_idx);
                    }

                    for row in query_result.rows.unwrap() {
                        let mut row_vec: IndexMap<String, Value> = IndexMap::new();

                        for index in &indexes {
                            let column = row.columns[index.to_owned()].as_ref();
                            let name = query_map[index.to_owned()].name.as_str().to_string();

                            let value = parse_cql_value(column);

                            row_vec.insert(name, value);
                        }

                        result.push(row_vec);
                    }

                    let mut query_result = Command {
                        hash: "".to_string(),
                        length: 0,
                        command: "select".to_string(),
                        keyspace: None,
                        table: None,
                        data: CommandData::SelectResponse(QueryResult {
                            result,
                            error: None,
                        }),
                        nonce: raw_command.nonce.clone(), // todo: do not clone
                    };

                    let string_query_data = serde_json::to_string(&query_result.data).unwrap();

                    query_result.length = string_query_data.len() + query_result.command.len();

                    query_result.hash = calculate_hash(
                        query_result.command.to_string()
                            + &query_result.length.to_string()
                            + &string_query_data,
                    );

                    let response = serde_json::to_string(&query_result).unwrap();

                    match write.lock().await.send(Message::Text(response)).await {
                        _ => {}
                    }
                }
                Err(error) => {
                    let query_result = Command {
                        hash: "".to_string(),
                        length: 0,
                        command: "select".to_string(),
                        keyspace: None,
                        table: None,
                        data: CommandData::SelectResponse(QueryResult {
                            result: Vec::new(),
                            error: Some(error.to_string()),
                        }),
                        nonce: raw_command.nonce.clone(),
                    };

                    let response = serde_json::to_string(&query_result).unwrap();

                    match write.lock().await.send(Message::Text(response)).await {
                        _ => {}
                    }
                }
            }
        }

        _ => {
            if *LOGGING.lock().await {
                println!("[Warn] A User sent an invalid command: {:?}", command);
            }
        }
    }
}
