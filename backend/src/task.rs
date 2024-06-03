use std::{collections::HashMap, fs, str::FromStr, sync::Mutex};

use rand::Rng;
use rocket::State;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::{database::Database, group::Group, login_info::{LoginInformation, LoginResult}, utils};

pub const TASK_ID_MAX: u128 = 4294967296u128; // 16^8, 2^32

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u128,

    pub title: String,
    pub description: String,
    pub species: Species,

    pub assigned: Vec<u128>
}
impl Task {
    pub fn save(db: &Database) {
        fs::write("data/tasks.json", serde_json::to_string_pretty(&db.tasks).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, Task> {
        serde_json::from_str(fs::read_to_string("data/tasks.json").unwrap().as_str()).unwrap()
    }

    pub fn create(db: &mut Database, group_id: u128, title: String, description: String, species: Species) {
        match db.groups.get_mut(&group_id) {
            Some(g) => {
                let id = utils::generate_id(db.tasks.keys().map(|k| *k).collect::<Vec<u128>>(), TASK_ID_MAX);
                g.tasks.push(id);
                db.tasks.insert(id, Task {
                    id,
                    title,
                    description,
                    assigned: vec![],
                    species
                });
                db.save();
            },
            None => {}
        }
    }

    pub fn delete(db: &mut Database, task_id: u128) {
        if db.tasks.contains_key(&task_id) {
            match Group::parent_of_task(&db, task_id).map_or(None, |i| db.groups.get_mut(&i)) {
                Some(g) => {
                    let indices = g.tasks.iter().enumerate().map(|(i, e)| (i, *e)).filter(|(_, t)| *t == task_id).collect::<Vec<(usize, u128)>>();
                    if !indices.is_empty() {
                        g.tasks.remove(indices[0].0);
                        db.tasks.remove(&task_id);
                        db.save();
                    }
                },
                None => {}
            }
        }
    }

    pub fn edit(db: &mut Database, task_id: u128, title: String, description: String) {
        match db.tasks.get_mut(&task_id) {
            Some(t) => {
                t.title = title;
                t.description = description;
                db.save();
            },
            None => {}
        }
    }


    pub fn assign(&mut self, user_id: u128, state: bool) {
        if state {
            self.assigned.push(user_id);
        } else {
            if self.assigned.contains(&user_id) {
                let target = self.assigned
                    .iter()
                    .enumerate()
                    .filter(|(_, u)| **u == user_id)
                    .map(|(u, _)| u.clone())
                    .collect::<Vec<usize>>()[0];
                self.assigned.remove(target);
            }
        }
    }    

    pub fn toggle_assign(&mut self, user_id: u128) {
        if self.assigned.contains(&user_id) {
            let target = self.assigned
                .iter()
                .enumerate()
                .filter(|(_, u)| **u == user_id)
                .map(|(u, _)| u.clone())
                .collect::<Vec<usize>>()[0];
            self.assigned.remove(target);
        } else {
            self.assigned.push(user_id);
        }
    }

    pub fn complete(&mut self, state: bool) {
        match self.species {
            Species::Task(_) => self.species = Species::Task(state),
            _ => {}
        }
    }

    pub fn toggle_complete(&mut self) {
        match self.species {
            Species::Task(s) => self.species = Species::Task(!s),
            _ => {}
        }
    }
}
#[derive(Serialize, Deserialize, Clone, EnumString)]
pub enum Species {
    #[strum(ascii_case_insensitive)]
    Task(bool),
    #[strum(ascii_case_insensitive)]
    Event
}

// #region api calls
#[post("/<group_id>/<title>/<description>/<raw_species>", data="<login>")]
pub fn create(db: &State<Mutex<Database>>, login: LoginInformation, group_id: u128, title: String, description: String, raw_species: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            let species = match Species::from_str(&raw_species) {
                Ok(i) => match i {
                    Species::Task(_) => Species::Task(false),
                    _ => Species::Event
                },
                Err(_) => Species::Event
            };
            Task::create(&mut db, group_id, utils::decode_uri(title), utils::decode_uri(description), species);
            utils::parse_response(Ok("success"))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<task_id>/<title>/<description>", data="<login>")]
pub fn edit(db: &State<Mutex<Database>>, login: LoginInformation, task_id: u128, title: String, description: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            Task::edit(&mut db, task_id, utils::decode_uri(title), utils::decode_uri(description));
            utils::parse_response(Ok("success"))
        }
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<task_id>", data="<login>")]
pub fn delete(db: &State<Mutex<Database>>, login: LoginInformation, task_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            Task::delete(&mut db, task_id);
            utils::parse_response(Ok("success"))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<task_id>/<user_id>/<state>", data="<login>")]
pub fn assign(db: &State<Mutex<Database>>, login: LoginInformation, task_id: u128, user_id: u128, state: bool) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.tasks.get_mut(&task_id) {
            Some(t) => {
                t.assign(user_id, state);
                db.save();
                utils::parse_response(Ok("success"))
            },
            None => utils::parse_response(Ok(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<task_id>/<user_id>", data="<login>")]
pub fn toggle_assign(db: &State<Mutex<Database>>, login: LoginInformation, task_id: u128, user_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.tasks.get_mut(&task_id) {
            Some(t) => {
                t.toggle_assign(user_id);
                db.save();
                utils::parse_response(Ok(""))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<task_id>/<state>", data="<login>")]
pub fn complete(db: &State<Mutex<Database>>, login: LoginInformation, task_id: u128, state: bool) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.tasks.get_mut(&task_id) {
            Some(t) => {
                t.complete(state);
                db.save();
                utils::parse_response(Ok("success"))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<task_id>", data="<login>")]
pub fn toggle_complete(db: &State<Mutex<Database>>, login: LoginInformation, task_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.tasks.get_mut(&task_id) {
            Some(t) => {
                t.toggle_complete();
                db.save();
                utils::parse_response(Ok("success"))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}
// #endregion
