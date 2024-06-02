use std::{collections::HashMap, sync::Mutex};

use rand::Rng;
use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{database::Database, login_info::{LoginInformation, LoginResult}, task::{self, Task}, utils};

pub const GROUP_ID_MAX: u128 = 4294967296u128; // 16^8, 2^32s

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,

    pub tasks: HashMap<u128, Task>
}
impl Group {
    // pub fn create_task(Task) {

    // }

    pub fn generate_task_id(&self) -> u128 {
        let fallback = self.tasks
            .keys()
            .max()
            .map_or(0, |i| i + 1);

        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            // try generate for 1k times, else, resort to fallback
            let candidate = rng.gen_range(0..task::TASK_ID_MAX);
            if self.tasks.contains_key(&candidate) {
                continue;
            }

            return candidate;
        }
        fallback
    }

    pub fn create_task(&mut self, title: String, description: String) {
        self.tasks.insert(self.generate_task_id(), Task {
            title,
            description,
            assigned: vec![]
        });
    }

    pub fn delete_task(&mut self, task_id: u128) {
        if self.tasks.contains_key(&task_id) {
            self.tasks.remove(&task_id);
        }
    }

    pub fn edit_task(&mut self, task_id: u128, title: String, description: String) {
        match self.tasks.get_mut(&task_id) {
            Some(t) => {
                t.title = title;
                t.description = description;
            },
            None => {}
        }
    }
}

// #region api calls
#[post("/<project_id>/<name>", data="<login>")]
pub fn create_group(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, name: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            db.projects.get_mut(&project_id).unwrap().create_group(utils::decode_uri(name));
            db.save();
            utils::parse_response(Ok("success".to_string()))
        },
        _ => utils::parse_response(Err(result))
    }    
}

#[post("/<project_id>/<group_id>", data="<login>")]
pub fn delete_group(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            db.projects.get_mut(&project_id).unwrap().delete_group(group_id);
            db.save();
            utils::parse_response(Ok("success".to_string()))
        },
        _ => utils::parse_response(Err(result))
    }    
}

#[post("/<project_id>/<group_id>/<name>", data="<login>")]
pub fn edit_group(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, name: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(_) => {
            db.projects.get_mut(&project_id).unwrap().edit_group(group_id, utils::decode_uri(name));
            db.save();
            utils::parse_response(Ok("success".to_string()))
        },
        _ => utils::parse_response(Err(result))
    }    
}
// #endregion

