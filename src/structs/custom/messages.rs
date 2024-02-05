use scylla::frame::value::CqlTimestamp;
use scylla::SerializeRow;
use scylla::SerializeCql;
use serde::{Deserialize, Serialize};

use super::udt::MainObject;
use super::udt::MainObjectUDT;

#[derive(Clone, Debug, SerializeRow, SerializeCql)]
pub struct MessagesCQL {
    pub message_id: String,
    pub author_id: String,
    pub content: Option<String>,
    pub allowed_mentions: i32,
    pub updated_date: Option<CqlTimestamp>,
    pub channel_id: String,
    pub nonce: Option<String>,
    pub bucket: String,
    pub flags: i32,
    pub mentions: Vec<String>,
    pub mention_roles: Vec<String>,
    pub mention_channels: Vec<String>,
    pub embeds: Vec<MainObjectUDT>,
    pub attachments: Vec<String>,
    pub replying_to: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Messages {
    pub message_id: String,
    pub author_id: String,
    pub content: Option<String>,
    pub allowed_mentions: i32,
    pub updated_date: Option<String>,
    pub channel_id: String,
    pub nonce: Option<String>,
    pub bucket: String,
    pub flags: i32,
    pub mentions: Vec<String>,
    pub mention_roles: Vec<String>,
    pub mention_channels: Vec<String>,
    pub embeds: Vec<MainObject>,
    pub attachments: Vec<String>,
    pub replying_to: Option<String>,
}