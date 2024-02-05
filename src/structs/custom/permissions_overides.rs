use scylla::SerializeRow;
use scylla::SerializeCql;
use serde::{Deserialize, Serialize};

use super::udt::BigintPair;
use super::udt::BigintPairUDT;

#[derive(Clone, Debug, SerializeRow, SerializeCql)]
pub struct PermissionOverridesCQL {
    pub permission_id: String,
    pub id: String,
    pub allow_: Vec<BigintPairUDT>,
    pub deny: Vec<BigintPairUDT>,
    #[scylla(rename = "type")]
    pub type_: i32,
    pub editable: bool,
    pub slowmode: i32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionOverrides {
    pub permission_id: String,
    pub id: String,
    pub allow_: Vec<BigintPair>,
    pub deny: Vec<BigintPair>,
    #[serde(rename = "type")]
    pub type_: i32,
    pub editable: bool,
    pub slowmode: i32
}