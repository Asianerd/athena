use std::{collections::HashMap, fs, sync::Mutex};

use rand::Rng;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{database::Database, login_info::{LoginInformation, LoginResult}, project::Project, task::{self, Task}, utils};

pub const GROUP_ID_MAX: u128 = 4294967296u128; // 16^8, 2^32s

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,

    pub tasks: Vec<u128>
}
impl Group {
    pub fn save(db: &Database) {
        fs::write("data/groups.json", serde_json::to_string_pretty(&db.groups).unwrap()).unwrap();
    }

    pub fn load() -> HashMap<u128, Group> {
        serde_json::from_str(fs::read_to_string("data/groups.json").unwrap().as_str()).unwrap()
    }

    pub fn parent_of_task(db: &Database, task_id: u128) -> Option<u128> {
        for (i, g) in &db.groups {
            if g.tasks.contains(&task_id) {
                return Some(*i);
            }
        }
        None
    }

    pub fn create(db: &mut Database, project_id: &u128, name: String) {
        match db.projects.get_mut(&project_id) {
            Some(p) => {
                let id = utils::generate_id(db.groups.keys().map(|i| *i).collect::<Vec<u128>>(), GROUP_ID_MAX);
                p.groups.push(id);
                db.groups.insert(id, Group {
                    name,
                    tasks: vec![]
                });
                db.save();
            },
            None => {}
        }
    }

    pub fn delete(db: &mut Database, group_id: u128) {
        if db.groups.contains_key(&group_id) {
            match Project::parent_of_group(db, group_id).map_or(None, |i| db.projects.get_mut(&i)) {
                Some(p) => {
                    let indices = p.groups.iter().enumerate().map(|(i, e)| (i, *e)).filter(|(_, g)| *g == group_id).collect::<Vec<(usize, u128)>>();
                    if !indices.is_empty() {
                        p.groups.remove(indices[0].0);
                        for t in db.groups.get(&group_id).unwrap().tasks.clone() {
                            Task::delete(db, t);
                        }
                        db.groups.remove(&group_id);
                        db.save();
                    }
                },
                None => {}
            }
        }
    }

    pub fn edit(db: &mut Database, group_id: u128, name: String) {
        match db.groups.get_mut(&group_id) {
            Some(g) => {
                g.name = name;
                db.save()
            },
            None => {}
        }
    }
}

// #region api calls
#[post("/<project_id>/<name>", data="<login>")]
pub fn create(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, name: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            Group::create(&mut db, &project_id, name);
            utils::parse_response(Ok("success"))
        },
        _ => utils::parse_response(Err(result))
    }
}

#[post("/<group_id>", data="<login>")]
pub fn delete(db: &State<Mutex<Database>>, login: LoginInformation, group_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            Group::delete(&mut db, group_id);
            utils::parse_response(Ok("success"))
        },
        _ => utils::parse_response(Err(result))
    }    
}

#[post("/<group_id>/<name>", data="<login>")]
pub fn edit(db: &State<Mutex<Database>>, login: LoginInformation, group_id: u128, name: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            Group::edit(&mut db, group_id, name);
            utils::parse_response(Ok("success"))
        },
        _ => utils::parse_response(Err(result))
    }    
}
// #endregion

