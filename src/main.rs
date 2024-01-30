use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::calculate_hash::calculate_hash;
use crate::structs::common::Command;

mod calculate_hash;
mod commands;
mod state;
mod structs;
mod util;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    println!("[Info] Server listening on: {}", addr);

    let users = Arc::new(Mutex::new(state::Store::new()));

    while let Ok((stream, _)) = listener.accept().await {
        // let stream = Arc::new(Mutex::new(stream));
        let (read, write) = stream.into_split();

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
    let user = Arc::new(Mutex::new(state::ClientState::new(false, "test", None)));
    let rnd_id = rand::random::<u32>().to_string();
    let ip = read.lock().await.peer_addr().unwrap();

    println!("[Info] New connection from: {} with id: {}", ip, rnd_id);

    users.lock().await.clients.insert(rnd_id, Arc::clone(&user));

    loop {
        let mut buffer = [0; 1024];

        let n = read.lock().await.read(&mut buffer).await.unwrap();

        if n == 0 {
            break;
        }

        let received_data = &buffer[..n];

        let json_str = String::from_utf8_lossy(received_data);

        // Split the received data by the newline character
        let commands: Vec<&str> = json_str.split('\n').collect();

        for command in commands {
            if command.is_empty() {
                continue;
            }

            // println!("[Info] Received command: {}", command);

            // Process each command separately
            match serde_json::from_str::<Command>(command) {
                Ok(command) => {
                    let hash = calculate_hash(
                        command.command.to_string()
                            + &command.length.to_string()
                            + &serde_json::to_string(&command.data).unwrap(),
                    );

                    if hash != command.hash {
                        println!("[Warn] Hashes do not match, dropping command");

                        println!("Received hash: {}", command.hash);
                        println!("Calculated hash: {}", hash);

                        println!("Command: {}", command.command);
                        println!("Length: {}", command.length);
                        println!("Data: {}", serde_json::to_string(&command.data).unwrap());

                        write
                            .lock()
                            .await
                            .write_all("Hashes do not match, dropping command".as_bytes())
                            .await
                            .unwrap();

                        continue;
                    }

                    let feature = handle_command(Arc::clone(&write), command, Arc::clone(&user));

                    tokio::spawn(feature);
                }
                Err(e) => {
                    let response = format!("Error: {}", e);

                    println!("[Warn] A User sent an invalid command: {}", command);

                    write
                        .lock()
                        .await
                        .write_all(response.as_bytes())
                        .await
                        .unwrap();
                }
            }
        }
    }
}

async fn handle_command(
    write: Arc<Mutex<OwnedWriteHalf>>,
    command: Command,
    user: Arc<Mutex<state::ClientState>>,
) {
    match command.command.as_str() {
        "connect" => {
            commands::connect::connect(Arc::clone(&write), command.data, user).await;
        }
        "select" => {
            commands::select::select(
                Arc::clone(&write),
                &command.data,
                user,
                &command.keyspace,
                &command.table,
                &command,
            )
            .await;
        }
        "insert" => {
            commands::insert::insert(
                Arc::clone(&write),
                &command.data,
                user,
                &command.keyspace,
                &command.table,
                &command,
            )
            .await;
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
