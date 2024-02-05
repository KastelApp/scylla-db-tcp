use scylla::macros::FromRow;
use scylla::SerializeCql;

#[derive(Clone, Debug, FromRow, SerializeCql)]
pub struct Bot {
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub avatar: String,
    pub summary: String,
    pub owner_id: String,
}
