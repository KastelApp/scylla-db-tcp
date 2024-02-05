use scylla::SerializeRow;
use scylla::SerializeCql;
use serde::{Deserialize, Serialize};

use super::udt::DmRecipients;
use super::udt::DmRecipientsUDT;

#[derive(Clone, Debug, SerializeRow, SerializeCql)]
pub struct DmsCQL {
    pub dm_id: String,
    pub recipients: Vec<DmRecipientsUDT>,
    pub channel_id: String,
    pub flags: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dms {
    pub dm_id: String,
    pub recipients: Vec<DmRecipients>,
    pub channel_id: String,
    pub flags: i32,
}