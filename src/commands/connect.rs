use futures_util::{stream::SplitSink, SinkExt};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    state,
    structs::common::{Command, CommandData, QueryResult},
    LOGGING,
};

pub async fn connect(
    write: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    command: CommandData,
    user: Arc<Mutex<state::ClientState>>,
) {
    let mut write = write.lock().await;
    let mut uu = user.lock().await;

    if uu.connected {
        let error = Command {
            command: "error".to_string(),
            data: CommandData::SelectResponse(QueryResult {
                error: Some("Already connected to scylla".to_string()),
                result: Vec::new(),
            }),
            keyspace: None,
            table: None,
            hash: "".to_string(),
            length: "".len(),
            nonce: None,
        };

        let response = serde_json::to_string(&error).unwrap();

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
        CommandData::Connect(connect_data) => {
            uu.keyspace = connect_data.keyspace;

            match scylla::SessionBuilder::new()
                .known_node(connect_data.contact_points[0].as_str())
                .user(
                    connect_data.credentials.username,
                    connect_data.credentials.password,
                )
                .build()
                .await
            {
                Ok(session) => {
                    uu.session = Some(Arc::new(Mutex::new(session)));
                    uu.connected = true;

                    let response = Command {
                        command: "connect".to_string(),
                        data: CommandData::ConnectResponse(crate::structs::connect::ConnectResponse {
                            result: "Connected to scylla".to_string(),
                            error: None,
                        }),
                        keyspace: None,
                        table: None,
                        hash: "".to_string(),
                        length: "".len(),
                        nonce: None,
                    };

                    match write.send(Message::Text(serde_json::to_string(&response).unwrap())).await {
                        // ? we don't care about if it succeeds or not
                        _ => {}
                    }
                }
                Err(error) => {
                    uu.connected = false;

                    let response = Command {
                        command: "connect".to_string(),
                        data: CommandData::ConnectResponse(crate::structs::connect::ConnectResponse {
                            result: "Failed to connect to scylla".to_string(),
                            error: Some(error.to_string()),
                        }),
                        keyspace: None,
                        table: None,
                        hash: "".to_string(),
                        length: "".len(),
                        nonce: None,
                    };

                    match write.send(Message::Text(serde_json::to_string(&response).unwrap())).await {
                        // ? we don't care about if it succeeds or not
                        _ => {}
                    }

                    match write.close().await {
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
