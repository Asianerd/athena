use std::{collections::HashMap, fs, sync::Mutex};

use rand::Rng;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{database::Database, group::{self, Group}, login_info::{LoginInformation, LoginResult}, user::User, utils};

pub const PROJECT_ID_MAX: u128 = 4294967296u128; // 16^8, 2^32

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,

    pub owner: Ownership,

    pub groups: HashMap<u128, Group>
}
impl Project {
    pub fn save(db: &Database) {
        fs::write("data/projects.json", serde_json::to_string_pretty(&db.projects).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, Project> {
        serde_json::from_str(fs::read_to_string("data/projects.json").unwrap().as_str()).unwrap()
    }

    pub fn generate_id(account_handler: &Database) -> u128 {
        let fallback = account_handler.projects
            .keys()
            .max()
            .map_or(0, |i| i + 1); // stack overflow if more than u128::MAX users

        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            // try generate for 1k times, else, resort to fallback
            let candidate = rng.gen_range(0..PROJECT_ID_MAX);
            if account_handler.users.contains_key(&candidate) {
                continue;
            }

            return candidate;
        }
        fallback
    }

    pub fn generate_group_id(&self) -> u128 {
        let fallback = self.groups
            .keys()
            .max()
            .map_or(0, |i| i + 1);

        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            // try generate for 1k times, else, resort to fallback
            let candidate = rng.gen_range(0..group::GROUP_ID_MAX);
            if self.groups.contains_key(&candidate) {
                continue;
            }

            return candidate;
        }
        fallback
    }

    pub fn create_group(&mut self, name: String) {
        self.groups.insert(self.generate_group_id(), Group {
            name,
            tasks: HashMap::new()
        });
    }

    pub fn delete_group(&mut self, group_id: u128) {
        if self.groups.contains_key(&group_id) {
            self.groups.remove(&group_id);
        }
    }

    pub fn edit_group(&mut self, group_id: u128, name: String) {
        match self.groups.get_mut(&group_id) {
            Some(g) => {
                g.name = name;
            },
            None => {}
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Ownership {
    User(u128),
    Team(u128)
}

// #region api calls
#[post("/<name>", data="<login>")]
pub fn create_project(db: &State<Mutex<Database>>, login: LoginInformation, name: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => {
            User::create_project(user_id, &mut db, utils::decode_uri(name));
            db.save();
            utils::parse_response(Ok("success".to_string()))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>", data="<login>")]
pub fn delete_project(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            User::delete_project(&mut db, project_id);
            db.save();
            utils::parse_response(Ok("success"))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>/<name>", data="<login>")]
pub fn edit_project(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, name: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            User::edit_project(&mut db, project_id, utils::decode_uri(name));
            db.save();
            utils::parse_response(Ok("success".to_string()))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>", data="<login>")]
pub fn fetch_project(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            utils::parse_response(Ok(
                db.projects.get(&project_id)
            ))
        },
        _ => utils::parse_response(Err(result))
    }
}
// #endregion
