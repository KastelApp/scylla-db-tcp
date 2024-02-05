use scylla::{macros::FromRow, SerializeCql};

#[derive(Clone, Debug, SerializeCql, FromRow)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub tag: String,
    pub avatar: String,
    pub password: String,
    pub phone_number: String,
    pub two_fa_secret: String,
    pub ips: Vec<String>,
    pub public_flags: String,
    pub flags: String,
    pub guilds: Vec<String>,
    pub global_nickname: String,
}