use scylla::SerializeCql;
use scylla::macros::FromRow;

#[derive(Clone, Debug, SerializeCql, FromRow)]
pub struct EmptyBuckets {
    pub channel_id: String,
    pub buckets: Vec<String>,
}
