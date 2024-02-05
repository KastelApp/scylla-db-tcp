use scylla::{frame::value::CqlTimestamp, SerializeCql};

#[derive(Clone, Debug, SerializeCql)]
pub struct Ban {
    pub guild_id: String,
    pub user_id: String,
    pub banner_id: String,
    pub reason: String,
    pub banned_date: CqlTimestamp,
    pub unban_date: CqlTimestamp,
}