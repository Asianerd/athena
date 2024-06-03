use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

use crate::{database::Database, project::{self, Project}};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u128,
    pub username: String,
}
impl User {
    pub fn save(db: &Database) {
        fs::write("data/users.json", serde_json::to_string_pretty(&db.users).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, User> {
        serde_json::from_str(fs::read_to_string("data/users.json").unwrap().as_str()).unwrap()
    }
}


// #region api calls

// #endregion
