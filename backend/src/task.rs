use std::sync::Mutex;

use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{database::Database, login_info::{LoginInformation, LoginResult}, utils};

pub const TASK_ID_MAX: u128 = 4294967296u128; // 16^8, 2^32

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,

    pub assigned: Vec<u128>
}
impl Task {
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
}

// #region api calls
#[post("/<project_id>/<group_id>/<title>/<description>", data="<login>")]
pub fn create_task(db: &State<Mutex<Database>>, login: LoginInformation, project_id: u128, group_id: u128, title: String, description: String) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    utils::parse_response(
        match result {
            LoginResult::Success(_) => match db.projects.get_mut(&project_id) {
                Some(p) => match p.groups.get_mut(&group_id) {
                    Some(g) => {
                        g.create_task(utils::decode_uri(title), utils::decode_uri(description));
                        db.save();
                        Ok("success")
                    },
                    None => Err("")
                },
                None => Err("")
            },
            _ => Err(result)
        }
    )
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
// #endregion
