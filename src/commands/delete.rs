use futures_util::{stream::SplitSink, SinkExt};
use indexmap::IndexMap;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    calculate_hash::calculate_hash,
    state,
    structs::{common::{Command, CommandData, QueryResult, Value}, update::UpdateResponse},
    util::{parse_cql_value::parse_cql_value, queries::{select_query, update_query}},
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
            let table = table.as_ref().unwrap();
            let keyspace = keyspace.as_ref();
            let user_keyspace = &user.keyspace;

           }

        _ => {
            if *LOGGING.lock().await {
                println!("[Warn] A User sent an invalid command: {:?}", command);
            }
        }
    }
}
