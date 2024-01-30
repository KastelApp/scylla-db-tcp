use crate::structs;

#[derive(Clone, Debug)]
pub struct Query {
    pub query: String,
    pub values: Vec<structs::Value>,
    pub map: Vec<String>
}


pub fn select_query(keyspace: String, table: String, data: structs::SelectData) -> Query {
    let mut query = String::from("SELECT ");

    let mut values: Vec<structs::Value> = Vec::new();

    let mut columns = Vec::new();

    let wc = &data.where_clause;

    for column in data.columns {
        columns.push(column);
    }

    if columns.len() == 0 {
        println!("[Warn] No columns specified, defaulting to *, this is not recommended!");

        query.push_str("*");
    }

    query.push_str(&columns.join(", "));

    query.push_str(" FROM ");

    query.push_str(keyspace.as_str());

    query.push_str(".");

    query.push_str(table.as_str());

    query.push_str(" WHERE ");

    let mut where_clause = Vec::new();

    for (key, value) in wc.clone() {
        where_clause.push(format!("{} = ?", key.clone()));

        values.push(value.clone());
    }

    query.push_str(&where_clause.join(" AND "));

    query.push_str(" LIMIT ");

    query.push_str(&data.limit.to_string());

    Query {
        query,
        values,
        map: columns
    }
}