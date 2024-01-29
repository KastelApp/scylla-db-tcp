use std::{borrow::Borrow, sync::Arc};

use tokio::{
    io::AsyncWriteExt,
    net::{tcp::OwnedWriteHalf, TcpStream},
    sync::Mutex,
};

use crate::{state, structs};

pub async fn connect(
    write: Arc<Mutex<OwnedWriteHalf>>,
    command: structs::CommandData,
    user: Arc<Mutex<state::ClientState>>,
) {
    let mut write = write.lock().await;
    let mut uu = user.lock().await;

    if uu.connected {
        let response = "Already connected to scylla";

        write.write_all(response.as_bytes()).await.unwrap();

        write.shutdown().await.unwrap();

        return;
    }

    uu.connected = true;

    match command {
        structs::CommandData::Connect(connect_data) => {
            uu.keyspace = connect_data.keyspace;

            uu.session = Some(Arc::new(Mutex::new(
                scylla::SessionBuilder::new()
                    .known_node(connect_data.contact_points[0].as_str())
                    .user(connect_data.credentials.username, connect_data.credentials.password)
                    .build()
                    .await
                    .unwrap(),
            )));

            let response = "Connected to scylla";

            write.write_all(response.as_bytes()).await.unwrap();
        }
        _ => {
            println!("Unknown command data: {:?}", command);
        }
    }
}