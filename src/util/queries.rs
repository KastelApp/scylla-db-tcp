use crate::structs::{common::Value, insert::InsertData, select::SelectData};

pub struct Query<'a> {
    pub query: String,
    pub values: Vec<&'a Value>,
    pub map: Vec<String>,
}

pub fn select_query<'a>(
    keyspace: &'a String,
    table: &'a String,
    data: &'a SelectData,
) -> Query<'a> {
    let mut query = String::from("SELECT ");

    let mut values: Vec<&Value> = Vec::new();

    let mut columns = Vec::new();

    let wc = &data.where_clause;

    for column in &data.columns {
        columns.push(column);
    }

    if columns.len() == 0 {
        query.push_str("*");
    }

    let column_joinable = columns
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    query.push_str(column_joinable.join(", ").as_str());
    query.push_str(" FROM ");
    query.push_str(keyspace.as_str());
    query.push_str(".");
    query.push_str(table.as_str());

    if wc.len() > 0 {
        query.push_str(" WHERE ");
    }

    let mut where_clause = Vec::new();

    for (key, value) in wc {
        where_clause.push(format!("{} = ?", key));

        values.push(value);
    }

    query.push_str(&where_clause.join(" AND "));

    // ? If its 0 wtf is the point of the query lol, so thats why we use this to determine if we should add a limit
    if data.limit > 0 {
        query.push_str(" LIMIT ");
        query.push_str(&data.limit.to_string());
    }

    println!("{}", query);

    Query {
        query: query.to_string(),
        values,
        map: column_joinable,
    }
}

pub fn insert_query<'a>(
    keyspace: &'a String,
    table: &'a String,
    data: &'a InsertData,
) -> Query<'a> {
    let mut query = String::from("");

    if data.if_not_exists.to_owned().unwrap_or(false) {
        query.push_str("INSERT INTO IF NOT EXISTS ");
    } else {
        query.push_str("INSERT INTO ");
    }

    let mut values: Vec<&Value> = Vec::new();

    query.push_str(keyspace.as_str());
    query.push_str(".");
    query.push_str(table.as_str());
    query.push_str(" (");

    let mut columns = Vec::new();

    for (key, value) in &data.columns {
        columns.push(key);

        values.push(value);
    }

    let column_joinable = columns
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    query.push_str(column_joinable.join(", ").as_str());
    query.push_str(") VALUES (");

    let mut value_placeholders = Vec::new();

    for _ in columns {
        value_placeholders.push("?");
    }

    query.push_str(&value_placeholders.join(", "));
    query.push_str(")");

    Query {
        query,
        values,
        map: column_joinable,
    }
}

pub fn raw_query<'a>(query: &'a String, limit: i32) -> Query<'a> {
    let mut query = query.to_string();

    let values: Vec<&Value> = Vec::new();

    if limit > 0 {
        query.push_str(" LIMIT ");
        query.push_str(&limit.to_string());
    }

    Query {
        query,
        values,
        map: Vec::new(),
    }
}
