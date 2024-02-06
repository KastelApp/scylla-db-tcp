use scylla::frame::value::CqlTimestamp;
use scylla::SerializeRow;
use scylla::SerializeCql;
use serde::{Deserialize, Serialize};

use super::udt::MemberTimeouts;
use super::udt::MemberTimeoutsUDT;

#[derive(Clone, Debug, SerializeRow, SerializeCql)]
pub struct GuildMembersCQL {
    pub guild_id: String,
    pub user_id: String,
    pub roles: Vec<String>,
    pub nickname: Option<String>,
    pub joined_at: CqlTimestamp,
    pub flags: i32,
    pub timeouts: Vec<MemberTimeoutsUDT>,
    pub guild_member_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildMembers {
    pub guild_id: String,
    pub user_id: String,
    pub roles: Vec<String>,
    pub nickname: Option<String>,
    pub joined_at: String,
    pub flags: i32,
    pub timeouts: Vec<MemberTimeouts>,
    pub guild_member_id: String,
}