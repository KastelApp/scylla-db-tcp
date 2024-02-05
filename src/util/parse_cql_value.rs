use scylla::frame::response::result::CqlValue;

use crate::structs::common::Value;

pub fn parse_cql_value(column: Option<&CqlValue>) -> Value {
    match column {
        Some(value) => match value {
            scylla::frame::response::result::CqlValue::Int(num) => {
                Value::Num(num.to_owned().into())
            }
            scylla::frame::response::result::CqlValue::Text(text) => Value::Str(text.to_owned()),
            scylla::frame::response::result::CqlValue::Boolean(boolean) => {
                Value::Bool(boolean.to_owned())
            }
            scylla::frame::response::result::CqlValue::BigInt(bigint) => {
                Value::Str(bigint.to_owned().to_string())
            }
            scylla::frame::response::result::CqlValue::List(list) => {
                let mut array = Vec::new();

                for value in list {
                    array.push(parse_cql_value(Some(value)));
                }

                Value::Array(array)
            }
            scylla::frame::response::result::CqlValue::Empty => Value::Null,
            scylla::frame::response::result::CqlValue::Blob(blob) => {
                Value::Str(String::from_utf8(blob.to_owned()).unwrap())
            }
            scylla::frame::response::result::CqlValue::Map(decimal) => {
                let mut map = Vec::new();

                for (key, value) in decimal {
                    map.push((parse_cql_value(Some(key)), parse_cql_value(Some(value))));
                }

                Value::Map(map)
            }
            scylla::frame::response::result::CqlValue::Timestamp(timestamp) => {
                let string = timestamp.0.to_owned();
                let native = chrono::NaiveDateTime::from_timestamp_millis(string);
                let datetime = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(native.unwrap(), chrono::Utc);

                Value::Date(datetime.to_rfc3339())
            }
            _ => {
                println!("[Error] Failed to parse CqlValue: {:?}", value);

                Value::Null
            }
        },
        None => Value::Null,
    }
}
