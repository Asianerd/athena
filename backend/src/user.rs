use std::{collections::HashMap, fs, str::FromStr, sync::Mutex};

use rocket::State;
use serde::{Deserialize, Serialize};

use crate::{database::Database, login_info::{LoginInformation, LoginResult}, project::{self, Ownership, Project}, team::Team, utils};

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

    pub fn fetch_teams(db: &Database, user_id: u128) -> Vec<Team> {
        db.teams.iter().filter(|(_, t)| t.members.contains_key(&user_id)).map(|(_, i)| i.clone()).collect::<Vec<Team>>()
    }
}


// #region api calls
#[post("/", data="<login>")]
pub fn fetch_teams(db: &State<Mutex<Database>>, login: LoginInformation) -> String {
    let mut db = db.lock().unwrap();
    let result = login.login(&mut db);
    match result {
        LoginResult::Success(user_id) => {
            utils::parse_response(Ok(User::fetch_teams(&db, user_id)))
        },
        _ => utils::parse_response(Err(result))
    }
}
// #endregion
