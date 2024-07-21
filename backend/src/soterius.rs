use std::{collections::HashMap, fs};

pub fn fetch(username: String) -> Option<(i64, String)> { // i64 instead of u128 due to sqlx
    let soterius: HashMap<String, (i64, String)> = serde_json::from_str(fs::read_to_string("../../data/users.json").unwrap().as_str()).unwrap();
    soterius.get(&username).map_or_else(|| None, |e| Some((e.0.clone(), e.1.clone())))
}
