use std::{collections::HashMap, fs, str::FromStr, sync::Mutex};

use rand::Rng;
use rocket::State;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::{database::Database, login_info::{LoginInformation, LoginResult}, utils};

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
        fs::write("data/tasks.json", serde_json::to_string_pretty(&db.projects).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, Task> {
        serde_json::from_str(fs::read_to_string("data/tasks.json").unwrap().as_str()).unwrap()
    }

    pub fn generate_id(db: &Database) -> u128 {
        let fallback = db.tasks
            .keys()
            .max()
            .map_or(0, |i| i + 1);

        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            // try generate for 1k times, else, resort to fallback
            let candidate = rng.gen_range(0..TASK_ID_MAX);
            if db.tasks.contains_key(&candidate) {
                continue;
            }

            return candidate;
        }
        fallback
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
    Task(bool),
    Event
}

// #region api calls
#[post("/<project_id>/<group_id>/<title>/<description>/<raw_species>", data="<login>")]
pub fn create_task(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, title: String, description: String, raw_species: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.projects.get_mut(&project_id) {
            Some(p) => match p.groups.get_mut(&group_id) {
                Some(g) => {
                    let species = match Species::from_str(&raw_species) {
                        Ok(i) => match i {
                            Species::Task(_) => Species::Task(false),
                            _ => Species::Event
                        },
                        Err(_) => Species::Event
                    };
                    g.create_task(utils::decode_uri(title), utils::decode_uri(description), species);
                    db.save();
                    utils::parse_response(Ok("success"))
                },
                None => utils::parse_response(Err(""))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>/<group_id>/<task_id>/<title>/<description>", data="<login>")]
pub fn edit_task(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, task_id: u128, title: String, description: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.projects.get_mut(&project_id) {
            Some(p) => match p.groups.get_mut(&group_id) {
                Some(g) => {
                    g.edit_task(task_id, utils::decode_uri(title), utils::decode_uri(description));
                    db.save();
                    utils::parse_response(Ok("success"))
                },
                None => utils::parse_response(Err(""))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>/<group_id>/<task_id>", data="<login>")]
pub fn delete_task(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, task_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.projects.get_mut(&project_id) {
            Some(p) => match p.groups.get_mut(&group_id) {
                Some(g) => {
                    g.delete_task(task_id);
                    db.save();
                    utils::parse_response(Ok("success"))
                },
                None => utils::parse_response(Err(""))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>/<group_id>/<task_id>/<user_id>/<state>", data="<login>")]
pub fn assign_task(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, task_id: u128, user_id: u128, state: bool) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.projects.get_mut(&project_id) {
            Some(p) => match p.groups.get_mut(&group_id) {
                Some(g) => match g.tasks.get_mut(&task_id) {
                    Some(t) => {
                        t.assign(user_id, state);
                        db.save();
                        utils::parse_response(Ok("success"))
                    },
                    None => utils::parse_response(Err(""))
                },
                None => utils::parse_response(Err(""))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>/<group_id>/<task_id>/<user_id>", data="<login>")]
pub fn toggle_assign_task(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, task_id: u128, user_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.projects.get_mut(&project_id) {
            Some(p) => match p.groups.get_mut(&group_id) {
                Some(g) => match g.tasks.get_mut(&task_id) {
                    Some(t) => {
                        t.toggle_assign(user_id);
                        db.save();
                        utils::parse_response(Ok("success"))
                    },
                    None => utils::parse_response(Err(""))
                },
                None => utils::parse_response(Err(""))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>/<group_id>/<task_id>/<state>", data="<login>")]
pub fn complete_task(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, task_id: u128, state: bool) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.projects.get_mut(&project_id) {
            Some(p) => match p.groups.get_mut(&group_id) {
                Some(g) => match g.tasks.get_mut(&task_id) {
                    Some(t) => {
                        t.complete(state);
                        db.save();
                        utils::parse_response(Ok("success"))
                    },
                    None => utils::parse_response(Err(""))
                },
                None => utils::parse_response(Err(""))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<project_id>/<group_id>/<task_id>", data="<login>")]
pub fn toggle_complete_task(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, task_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => match db.projects.get_mut(&project_id) {
            Some(p) => match p.groups.get_mut(&group_id) {
                Some(g) => match g.tasks.get_mut(&task_id) {
                    Some(t) => {
                        t.toggle_complete();
                        db.save();
                        utils::parse_response(Ok("success"))
                    },
                    None => utils::parse_response(Err(""))
                },
                None => utils::parse_response(Err(""))
            },
            None => utils::parse_response(Err(""))
        },
        _ => utils::parse_response(Err(result))
    }
}
// #endregion
