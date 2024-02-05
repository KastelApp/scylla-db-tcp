use scylla::SerializeRow;
use scylla::SerializeCql;
use serde::{Deserialize, Serialize};

use super::udt::BigintPair;
use super::udt::BigintPairUDT;

#[derive(Clone, Debug, SerializeRow, SerializeCql)]
pub struct RolesCQL {
    pub role_id: String,
    pub guild_id: String,
    pub name: String,
    pub allowed_age_restricted: bool,
    pub deleteable: bool,
    pub mentionable: bool,
    pub hoisted: bool,
    pub color: i32,
    pub permissions: Vec<BigintPairUDT>,
    pub position: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Roles {
    pub role_id: String,
    pub guild_id: String,
    pub name: String,
    pub allowed_age_restricted: bool,
    pub deleteable: bool,
    pub mentionable: bool,
    pub hoisted: bool,
    pub color: i32,
    pub permissions: Vec<BigintPair>,
    pub position: i32,
}