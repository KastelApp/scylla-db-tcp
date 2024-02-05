use futures_util::{stream::SplitSink, SinkExt};
use indexmap::IndexMap;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    calculate_hash, state, structs::
        common::{Command, CommandData, QueryResult, Value}, util::{parse_cql_value::parse_cql_value, queries::raw_query}, LOGGING
};

pub async fn raw(
    write: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    command: &CommandData,
    user: Arc<Mutex<state::ClientState>>,
    _: &Option<String>,
    _: &Option<String>,
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
            type_: None,
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
        CommandData::Raw(raw_data) => {
            let session = user.session.as_ref().unwrap().lock().await;

            let query = raw_query(&raw_data.query, raw_data.limit.to_owned().unwrap_or(0));

            match session.query(query.query, &raw_data.values).await {
                Ok(query_result) => {
                    let mut result = Vec::new();

                    let mut indexes = Vec::new();

                    let query_map = &query_result.col_specs;

                    for value in query_map {
                        let (value_idx, _) =
                            query_result.get_column_spec(value.name.as_str()).unwrap();

                        indexes.push(value_idx);
                    }

                    for row in query_result.rows.unwrap_or(Vec::new()) {
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
                        command: "raw".to_string(),
                        keyspace: None,
                        table: None,
                        data: CommandData::SelectResponse(QueryResult {
                            result,
                            error: None,
                        }),
                        nonce: raw_command.nonce.clone(), // todo: do not clone
                        type_: None,
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

                Err(e) => {
                    let mut response = Command {
                        command: "raw".to_string(),
                        data: CommandData::SelectResponse(QueryResult {
                            error: Some(e.to_string()),
                            result: Vec::new(),
                        }),
                        keyspace: None,
                        table: None,
                        hash: "".to_string(),
                        length: "".len(),
                        nonce: None,
                        type_: None,
                    };

                    let response_string = serde_json::to_string(&response).unwrap();

                    response.length = response_string.len() + response.command.len();

                    response.hash = calculate_hash(
                        response.command.to_string()
                            + &response.length.to_string()
                            + &response_string,
                    );

                    let response = serde_json::to_string(&response).unwrap();

                    let mut write = write.lock().await;

                    match write.send(Message::Text(response)).await {
                        // ? we don't care about if it succeeds or not
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
