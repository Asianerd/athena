use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub name: String,

    pub members: HashMap<u128, Permissions>
}
impl Team {
    pub fn save(db: &Database) {
        fs::write("data/teams.json", serde_json::to_string_pretty(&db.teams).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, Team> {
        serde_json::from_str(fs::read_to_string("data/teams.json").unwrap().as_str()).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub enum Permissions {
    Admin,

    Editor,
    Viewer,

    None
}
