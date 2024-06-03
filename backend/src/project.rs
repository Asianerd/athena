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

    pub groups: Vec<u128>
}
impl Project {
    pub fn save(db: &Database) {
        fs::write("data/projects.json", serde_json::to_string_pretty(&db.projects).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, Project> {
        serde_json::from_str(fs::read_to_string("data/projects.json").unwrap().as_str()).unwrap()
    }

    pub fn parent_of_group(db: &Database, group_id: u128) -> Option<u128> {
        for (i, p) in &db.projects {
            if p.groups.contains(&group_id) {
                return Some(*i);
            }
        }
        None
    }

    pub fn create(db: &mut Database, user_id: u128, name: String) {
        let id = utils::generate_id(db.projects.keys().map(|i| *i).collect::<Vec<u128>>(), PROJECT_ID_MAX);
        db.projects.insert(id, Project {
            name,
            owner: Ownership::User(user_id),
            groups: vec![]
        });
        db.save();
    }

    pub fn delete(db: &mut Database, project_id: u128) {
        if db.projects.contains_key(&project_id) {
            for g in db.projects.get(&project_id).unwrap().groups.clone() {
                Group::delete(db, g);
            }
            db.projects.remove(&project_id);
            db.save();
        }
    }

    pub fn edit(db: &mut Database, project_id: u128, name: String) {
        match db.projects.get_mut(&project_id) {
            Some(p) => {
                p.name = name;
                db.save();
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
pub fn create(db: &State<Mutex<Database>>, login: LoginInformation, name: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => {
            Project::create(&mut db, user_id, utils::decode_uri(name));
            utils::parse_response(Ok("success".to_string()))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>", data="<login>")]
pub fn delete(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            Project::delete(&mut db, project_id);
            utils::parse_response(Ok("success"))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>/<name>", data="<login>")]
pub fn edit(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, name: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            Project::edit(&mut db, project_id, utils::decode_uri(name));
            utils::parse_response(Ok("success".to_string()))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>", data="<login>")]
pub fn fetch(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128) -> String {
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
