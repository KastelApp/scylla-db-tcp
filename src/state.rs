use std::{collections::HashMap, sync::Arc};
use scylla::Session;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct ClientState {
    pub connected: bool,
    pub keyspace: String,
    pub session: Option<Arc<Mutex<Session>>>,
}

impl ClientState {
    pub fn new(connected: bool, keyspace: &str, session: Option<Session>) -> Self {
        let session = if connected {
            Some(Arc::new(Mutex::new(session.unwrap())))
        } else {
            None
        };

        Self {
            connected,
            keyspace: keyspace.to_string(),
            session,
        }
    }
}

#[derive(Debug)]
pub struct Store {
    pub clients: HashMap<String, Arc<Mutex<ClientState>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}