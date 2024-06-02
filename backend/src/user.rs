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

    pub fn create_project(user_id: u128, db: &mut Database, name: String) {
        let id = Project::generate_id(&db);
        db.projects.insert(id, Project {
            name,
            owner: project::Ownership::User(user_id),
            groups: HashMap::new()
        });
    }

    pub fn delete_project(db: &mut Database, project_id: u128) {
        if db.projects.contains_key(&project_id) {
            db.projects.remove(&project_id);
        }
    }

    pub fn edit_project(db: &mut Database, project_id: u128, name: String) {
        match db.projects.get_mut(&project_id) {
            Some(p) => {
                p.name = name;
            },
            None => {}
        }
    }
}


// #region api calls

// #endregion
