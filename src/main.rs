use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

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
        tokio::spawn(handle_connection(stream, Arc::clone(&users)));
    }
}

async fn handle_connection(raw_stream: TcpStream, users: Arc<Mutex<state::Store>>) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let ip = ws_stream.get_ref().peer_addr().unwrap();
    let user = Arc::new(Mutex::new(state::ClientState::new(false, "test", None)));
    let rnd_id = rand::random::<u32>().to_string();

    let (outgoing, incoming) = ws_stream.split();

    let outgoing = Arc::new(Mutex::new(outgoing));
    let incoming = Arc::new(Mutex::new(incoming));

    println!("[Info] New connection from: {} with id: {}", ip, rnd_id);

    users
        .lock()
        .await
        .clients
        .insert(rnd_id.clone(), Arc::clone(&user));

    let mut incoming = incoming.lock().await;

    while let Some(msg) = incoming.next().await {
        let msg = msg.unwrap();

        match msg {
            Message::Text(text) => match serde_json::from_str::<Command>(&text) {
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

                        outgoing
                            .lock()
                            .await
                            .send(Message::Text(
                                "Hashes do not match, dropping command".to_string(),
                            ))
                            .await
                            .unwrap();

                        continue;
                    }

                    let feature = handle_command(Arc::clone(&outgoing), command, Arc::clone(&user));

                    tokio::spawn(feature);
                }
                Err(e) => {
                    let response = format!("Error: {}", e);

                    println!("[Warn] A User sent an invalid command: {}", text);

                    outgoing
                        .lock()
                        .await
                        .send(Message::Text(response))
                        .await
                        .unwrap();
                }
            },
            Message::Close(_) => {
                println!("[Info] Received close");

                users.lock().await.clients.remove(&rnd_id);
            }
            _ => {
                println!("[Warn] Received unknown message");
            }
        }
    }
}

async fn handle_command(
    write: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
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
        _ => {
            println!("Unknown command: {:?}", command);
        }
    }
}
