use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialsData {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectData {
    #[serde(rename = "contactPoints")]
    pub contact_points: Vec<String>,
    #[serde(rename = "localDataCenter")]
    pub local_data_center: String,
    pub credentials: CredentialsData,
    pub keyspace: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectResponse {
    pub result: String,
    pub error: Option<String>,
}