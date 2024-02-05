use scylla::SerializeRow;
use scylla::SerializeCql;
use serde::{Deserialize, Serialize};

use super::udt::FriendsInteractions;
use super::udt::FriendsInteractionsUDT;

#[derive(Clone, Debug, SerializeRow, SerializeCql)]
pub struct FriendsCQL {
    pub friend_id: String,
    pub interactions: Vec<FriendsInteractionsUDT>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Friends {
    pub friend_id: String,
    pub interactions: Vec<FriendsInteractions>
}