use std::sync::Arc;

use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::Mutex};

use crate::{
    state,
    structs::common::{Command, CommandData, QueryResult},
};

pub async fn connect(
    write: Arc<Mutex<OwnedWriteHalf>>,
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

        write.write_all(response.as_bytes()).await.unwrap();

        write.shutdown().await.unwrap();

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

                    write.write_all(response.as_bytes()).await.unwrap();
                }
                Err(error) => {
                    uu.connected = false;

                    let response = format!("Failed to connect to scylla: {}", error);

                    write.write_all(response.as_bytes()).await.unwrap();

                    write.shutdown().await.unwrap();
                }
            }
        }
        _ => {
            println!("Unknown command data: {:?}", command);
        }
    }
}
