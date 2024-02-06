use scylla::{frame::value::CqlTimestamp, FromUserType, SerializeCql};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct TokensTypeUDT { // ? The CQL Type
    pub token_: String,
    pub created_date: CqlTimestamp,
    pub ip: String,
    pub flags: i32,
    pub token_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenType { // ! The Raw Type
    pub token_: String,
    pub created_date: String,
    pub ip: String,
    pub flags: i32,
    pub token_id: String,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct MentionsTypeUDT { // ? The CQL Type
    pub message_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentionsType { // ! The Raw Type
    pub message_id: String,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct GuildOrderTypeUDT { // ? The CQL Type
    pub guild_id: String,
    pub position: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildOrderType { // ! The Raw Type
    pub guild_id: String,
    pub position: i32,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct DmRecipientsUDT { // ? The CQL Type
    pub user_id: String,
    pub flags: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DmRecipients { // ! The Raw Type
    pub user_id: String,
    pub flags: i32,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct FriendsInteractionsUDT { // ? The CQL Type
    pub user_id: String,
    pub target_id: String,
    pub target_nickname: String,
    pub flags: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FriendsInteractions { // ! The Raw Type
    pub user_id: String,
    pub target_id: String,
    pub target_nickname: String,
    pub flags: i32,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct MemberTimeoutsUDT { // ? The CQL Type
    pub channel_id: String,
    pub timeout_until: CqlTimestamp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemberTimeouts { // ! The Raw Type
    pub channel_id: String,
    pub timeout_until: String,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct AuthorUDT { // ? The CQL Type
    pub name: String,
    pub iconurl: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author { // ! The Raw Type
    pub name: String,
    pub iconurl: String,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct FooterUDT { // ? The CQL Type
    pub text: String,
    pub timestamp_: CqlTimestamp,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Footer { // ! The Raw Type
    pub text: String,
    pub timestamp: String,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct FieldUDT { // ? The CQL Type
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field { // ! The Raw Type
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct MainObjectUDT { // ? The CQL Type
    pub title: String,
    pub description: String,
    pub url: String,
    pub color: String,
    pub author: AuthorUDT,
    pub footer: FooterUDT,
    pub fields: Vec<FieldUDT>,
    pub thumbnail_url: String,
    pub image_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MainObject { // ! The Raw Type
    pub title: String,
    pub description: String,
    pub url: String,
    pub color: String,
    pub author: Author,
    pub footer: Footer,
    pub fields: Vec<Field>,
    pub thumbnail_url: String,
    pub image_url: String,
}

#[derive(Clone, Debug, FromUserType, SerializeCql)]
pub struct BigintPairUDT { // ? The CQL Type
    pub first: i64,
    pub second: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BigintPair { // ! The Raw Type
    pub first: String,
    pub second: String,
}