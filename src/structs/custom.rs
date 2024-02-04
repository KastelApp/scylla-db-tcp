use chrono::Utc;
use scylla::{frame::value::CqlTimestamp, SerializeCql, SerializeRow};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, SerializeRow, SerializeCql)]
pub struct TokensTypeUDT { // name of the struct does NOT have to much
    pub token_: String,
    pub created_date: CqlTimestamp, // Or some other type that serializes as Timestamp: chrono::DateTime<Utc>, time::OffsetDateTime
    pub ip: String,
    pub flags: i32,
    pub token_id: String,
}

/*

CREATE TYPE IF NOT EXISTS mentions_type (
	message_id text,
);

CREATE TYPE IF NOT EXISTS guild_order_type (
	guild_id text,
	position int,
);

CREATE TYPE IF NOT EXISTS dm_recipients (
	user_id text,
	flags int,
);

CREATE TYPE IF NOT EXISTS friends_interactions (
	user_id text,
	target_id text,
	target_nickname text,
	flags int,
);

CREATE TYPE IF NOT EXISTS member_timeouts (
	channel_id text,
	timeout_until timestamp,
);

CREATE TYPE IF NOT EXISTS author (
    name text,
    iconurl text
);

CREATE TYPE IF NOT EXISTS footer (
    text text,
    timestamp_ timestamp
);

CREATE TYPE IF NOT EXISTS field (
    title text,
    description text
);

CREATE TYPE IF NOT EXISTS main_object (
    title text,
    description text,
    url text,
    color text,
    author frozen<author>,
    footer frozen<footer>,
    fields list<frozen<field>>,
    thumbnail_url text,
    image_url text
);

CREATE TYPE IF NOT EXISTS bigint_pair (
  first bigint,
  second bigint
);

*/

#[derive(SerializeRow, SerializeCql)]
pub struct MentionsTypeUDT {
    pub message_id: String,
}

#[derive(SerializeRow, SerializeCql)]
pub struct GuildOrderTypeUDT {
    pub guild_id: String,
    pub position: i32,
}

#[derive(SerializeRow, SerializeCql)]
pub struct DmRecipientsUDT {
    pub user_id: String,
    pub flags: i32,
}

#[derive(SerializeRow, SerializeCql)]
pub struct FriendsInteractionsUDT {
    pub user_id: String,
    pub target_id: String,
    pub target_nickname: String,
    pub flags: i32,
}

#[derive(SerializeRow, SerializeCql)]
pub struct MemberTimeoutsUDT {
    pub channel_id: String,
    pub timeout_until: CqlTimestamp,
}

#[derive(SerializeRow, SerializeCql)]
pub struct AuthorUDT {
    pub name: String,
    pub iconurl: String,
}

#[derive(SerializeRow, SerializeCql)]
pub struct FooterUDT {
    pub text: String,
    pub timestamp_: CqlTimestamp,
}

#[derive(SerializeRow, SerializeCql)]
pub struct FieldUDT {
    pub title: String,
    pub description: String,
}

#[derive(SerializeRow, SerializeCql)]
pub struct MainObjectUDT {
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

#[derive(SerializeRow, SerializeCql)]
pub struct BigintPairUDT {
    pub first: i64,
    pub second: i64,
}