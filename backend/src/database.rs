use std::{collections::HashMap, sync::Mutex};

use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{group::Group, project::Project, task::Task, team::Team, user::User};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub users: HashMap<u128, User>,
    pub teams: HashMap<u128, Team>,

    pub projects: HashMap<u128, Project>,
    pub groups: HashMap<u128, Group>,
    pub tasks: HashMap<u128, Task>
}
impl Database {
    pub fn save(&self) {
        User::save(&self);
        Team::save(&self);
        Project::save(&self);
    }

    pub fn load() -> Database {
        let result = Database {
            users: User::load(),
            teams: Team::load(),
            projects: Project::load(),
            groups: Group::load(),
            tasks: Task::load()
        };

        result
    }

    pub fn fetch_user_id(&self, username: &String) -> Option<u128> {
        for (i, u) in &self.users {
            if *username == u.username {
                return Some(*i);
            }
        }
        None
    }
}

// #region api calls
#[get("/")]
pub fn save(db: &State<Mutex<Database>>) -> String {
    let db = db.lock().unwrap();
    db.save();
    "success".to_string()
}

#[get("/")]
pub fn load(db: &State<Mutex<Database>>) -> String {
    let mut db = db.lock().unwrap();
    *db = Database::load();
    "success".to_string()
}

#[get("/")]
pub fn debug(db: &State<Mutex<Database>>) -> String {
    let db = db.lock().unwrap();
    serde_json::to_string_pretty(&db.users).unwrap()
}
// #endregion
