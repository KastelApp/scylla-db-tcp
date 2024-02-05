use scylla::SerializeRow;
use scylla::SerializeCql;
use serde::{Deserialize, Serialize};

use super::udt::{GuildOrderTypeUDT, MentionsTypeUDT, TokensTypeUDT, TokenType, MentionsType, GuildOrderType};

#[derive(Clone, Debug, SerializeRow, SerializeCql)]
pub struct CqlSettings {
    pub user_id: String,
    pub status: i32,
    pub custom_status: Option<String>,
    pub bio: Option<String>,
    pub tokens: Vec<TokensTypeUDT>,
    pub theme: String,
    pub language: String,
    pub privacy: i32,
    pub mentions: Vec<MentionsTypeUDT>,
    pub max_guilds: i32,
    pub max_file_upload_size: i32,
    pub guild_order: Vec<GuildOrderTypeUDT>,
    pub allowed_invites: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub user_id: String,
    pub status: i32,
    pub custom_status: Option<String>,
    pub bio: Option<String>,
    pub tokens: Vec<TokenType>,
    pub theme: String,
    pub language: String,
    pub privacy: i32,
    pub mentions: Vec<MentionsType>,
    pub max_guilds: i32,
    pub max_file_upload_size: i32,
    pub guild_order: Vec<GuildOrderType>,
    pub allowed_invites: i32,
}