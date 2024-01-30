use std::collections::HashMap;

use crate::structs;

pub fn map_to_values(data: HashMap<String, structs::Value>) -> Vec<structs::Value> {
    let mut array = Vec::new();

    for (_, value) in data {
        array.push(value);
    }

    array
}

pub fn map_to_keys(data: HashMap<String, structs::Value>) -> Vec<String> {
    let mut array = Vec::new();

    for (key, _) in data {
        array.push(key);
    }

    array
}