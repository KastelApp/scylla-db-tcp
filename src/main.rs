use anyhow::anyhow;
use std::borrow::Borrow;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

mod commands;
mod state;
mod structs;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    println!("Server listening on: {}", addr);

    let users = Arc::new(Mutex::new(state::Store::new()));

    while let Ok((stream, _)) = listener.accept().await {
        // let stream = Arc::new(Mutex::new(stream));
        let (read, write) = stream.into_split();
        let users = users.clone();

        let read = Arc::new(Mutex::new(read));
        let write = Arc::new(Mutex::new(write));

        tokio::spawn(handle_connection(
            Arc::clone(&read),
            Arc::clone(&write),
            Arc::clone(&users),
        ));
    }
}

async fn handle_connection(
    read: Arc<Mutex<OwnedReadHalf>>,
    write: Arc<Mutex<OwnedWriteHalf>>,
    users: Arc<Mutex<state::Store>>,
) {
    let mut buffer = [0; 1024];

    let user = Arc::new(Mutex::new(state::ClientState::new(false, "test", None)));

    let rnd_id = rand::random::<u32>().to_string();

    users
        .lock()
        .await
        .clients
        .insert(rnd_id.clone(), Arc::clone(&user));

    while let Ok(n) = read.lock().await.read(&mut buffer).await {
        if n == 0 {
            break;
        }

        let received_data = &buffer[..n];
        let json_str = String::from_utf8_lossy(received_data);
        let command: structs::Command = serde_json::from_str(&json_str).unwrap();

        handle_command(Arc::clone(&write), command, Arc::clone(&user)).await;
    }
}

async fn handle_command(
    write: Arc<Mutex<OwnedWriteHalf>>,
    command: structs::Command,
    user: Arc<Mutex<state::ClientState>>,
) {
    match command.command.as_str() {
        "connect" => {
            commands::connect::connect(Arc::clone(&write), command.data, user).await;
        }
        "select" => {
            // commands::select::select(command.data, user).await;
        }
        "insert" => {
            // commands::insert::insert(command.data, user).await;
        }
        "test" => {
            if !user.lock().await.connected {
                let response = "Not connected to scylla";

                write
                    .lock()
                    .await
                    .write_all(response.as_bytes())
                    .await
                    .unwrap();

                return;
            }

            let query_result = user
                .lock()
                .await
                .session
                .as_ref()
                .unwrap()
                .lock()
                .await
                .query("SELECT user_id from kstltest.users", &[])
                .await
                .unwrap();

            let (usr_id_idx, _) = query_result.get_column_spec("user_id").unwrap();

            for row in query_result.rows.unwrap() {
                // user_id column is a text column (user_id text) (see: https://github.com/KastelApp/CqlTables/blob/master/Tables/UserSchema.cql#L2)
                let user_id = row.columns[usr_id_idx].as_ref().unwrap().as_text().unwrap();

                write
                    .lock()
                    .await
                    .write_all(user_id.as_bytes())
                    .await
                    .unwrap();
            }
        }
        _ => {
            println!("Unknown command: {:?}", command);
        }
    }
}
