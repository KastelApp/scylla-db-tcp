use futures_util::{stream::SplitSink, SinkExt};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    state,
    structs::common::{Command, CommandData, QueryResult},
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

    uu.connected = true;

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

                    let response = "Connected to scylla";

                    match write.send(Message::Text(response.to_string())).await {
                        // ? we don't care about if it succeeds or not
                        _ => {}
                    }
                }
                Err(error) => {
                    uu.connected = false;

                    let response = format!("Failed to connect to scylla: {}", error);

                    match write.send(Message::Text(response)).await {
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
            println!("Unknown command data: {:?}", command);
        }
    }
}
