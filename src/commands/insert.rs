use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

use crate::{
    calculate_hash, state,
    structs::{
        common::{Command, CommandData, QueryResult},
        insert::InsertResponse,
    },
    util::queries::insert_query,
};

pub async fn insert(
    write: Arc<Mutex<OwnedWriteHalf>>,
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

        write.write_all(response.as_bytes()).await.unwrap();

        write.shutdown().await.unwrap();

        return;
    }

    match command {
        CommandData::Insert(insert_data) => {
            let session = user.session.as_ref().unwrap().lock().await;

            let table = table.as_ref().unwrap();
            let keyspace = keyspace.as_ref();
            let user_keyspace = &user.keyspace;

            let query = insert_query(
                &keyspace.to_owned().unwrap_or(user_keyspace),
                table,
                insert_data,
            );

            match session.query(query.query, query.values).await {
                Ok(_) => {
                    let mut response = Command {
                        command: "insert".to_string(),
                        data: CommandData::InsertResponse(InsertResponse {
                            error: None,
                            success: true,
                        }),
                        keyspace: None,
                        table: None,
                        hash: "".to_string(),
                        length: "".len(),
                        nonce: raw_command.nonce.clone(), // todo: do not clone
                    };

                    let string_response = serde_json::to_string(&response.data).unwrap();

                    response.length = string_response.len() + response.command.len();

                    response.hash = calculate_hash(
                        response.command.to_string()
                            + &response.length.to_string()
                            + &string_response,
                    );

                    let response = serde_json::to_string(&response).unwrap();

                    let mut write = write.lock().await;

                    write.write_all(response.as_bytes()).await.unwrap();
                }
                Err(error) => {
                    let response = Command {
                        command: "insert".to_string(),
                        data: CommandData::InsertResponse(InsertResponse {
                            error: Some(error.to_string()),
                            success: false,
                        }),
                        keyspace: None,
                        table: None,
                        hash: "".to_string(),
                        length: "".len(),
                        nonce: None,
                    };

                    let response = serde_json::to_string(&response).unwrap();

                    let mut write = write.lock().await;

                    write.write_all(response.as_bytes()).await.unwrap();
                }
            }
        }

        _ => {
            println!("[Warn] Unknown command data: {:?}", command);
        }
    }
}
