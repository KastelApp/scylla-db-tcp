use futures_util::{stream::SplitSink, SinkExt};
use indexmap::IndexMap;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    calculate_hash::calculate_hash,
    state,
    structs::{common::{Command, CommandData, QueryResult, Value}, delete::DeleteResponse, update::UpdateResponse},
    util::{parse_cql_value::parse_cql_value, queries::{delete_query, select_query, update_query}},
    LOGGING,
};

pub async fn delete(
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
        CommandData::Delete(delete_data) => {
            let session = user.session.as_ref().unwrap().lock().await;

            let table = table.as_ref().unwrap();
            let keyspace = keyspace.as_ref();
            let user_keyspace = &user.keyspace;

            let query = delete_query(&keyspace.to_owned().unwrap_or(user_keyspace), table, delete_data);

            match session.query(query.query, query.values).await {
                Ok(query_result) => {

                    let delete_response = Command {
                        hash: "".to_string(),
                        length: 0,
                        command: "delete".to_string(),
                        data: CommandData::DeleteResponse(DeleteResponse {
                            error: None,
                            success: true
                        }),
                        keyspace: None,
                        table: None,
                        nonce: raw_command.nonce.clone(), // todo: do not clone
                        type_: None,
                    };

                    let response = serde_json::to_string(&delete_response).unwrap();

                    let mut write = write.lock().await;

                    match write.send(Message::Text(response)).await {
                        // ? we don't care about if it succeeds or not
                        _ => {}
                    }
                }

                Err(e) => {
                    let error = Command {
                        hash: "".to_string(),
                        length: 0,
                        command: "delete".to_string(),
                        data: CommandData::DeleteResponse(DeleteResponse {
                            error: Some(e.to_string()),
                            success: false
                        }),
                        keyspace: None,
                        table: None,
                        nonce: raw_command.nonce.clone(), // todo: do not clone
                        type_: None,
                    };

                    let response = serde_json::to_string(&error).unwrap();

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
