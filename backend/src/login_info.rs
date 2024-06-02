use std::collections::HashMap;

use rocket::http::Status;
use rocket::request::Request;
use rocket::data::{self, Data, FromData};
use rocket::outcome::Outcome;
use serde::{Deserialize, Serialize};

use crate::{database::Database, soterius, user::User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginInformation {
    pub username: String,
    pub password: String
}
impl LoginInformation {
    // handles anything to do with password or logging in
    pub fn login(&self, account_handler: &mut Database) -> LoginResult {
        // check soterius if it exists first

        // if exists:
        //      if exists in aurum -> get user id and proceed
        //      if not exists in aurum -> generate new user id and proceed
        // if doesnt exist -> return username no exist

        match soterius::fetch(self.username.clone()) {
            Some((user_id, password)) => {
                if password != self.password {
                    return LoginResult::PasswordWrong;
                }

                let athena_lookup = account_handler.fetch_user_id(&self.username);
                if athena_lookup.is_none() {
                    account_handler.users.insert(user_id, User {
                        id: user_id,
                        username: self.username.clone()
                    });
                }

                return LoginResult::Success(user_id);
            },
            None => LoginResult::UsernameNoExist
        }
    }
}

#[rocket::async_trait]
impl<'l> FromData<'l> for LoginInformation {
    type Error = LoginInfoParseError;

    async fn from_data(_req: &'l Request<'_>, mut data: Data<'l>) -> data::Outcome<'l, Self> {
        let result = data.peek(512).await.to_vec();

        if result.is_empty() {
            // return OutCome::Error
            return Outcome::Error((
                Status::Ok,
                LoginInfoParseError::Empty
            ))
        }
        
        let result = result.iter().map(|x| (x.clone()) as char).collect::<String>();
        let result: HashMap<String, String> = serde_json::from_str(result.as_str()).unwrap();

        Outcome::Success(LoginInformation {
            username: result.get("username").unwrap().clone(),
            password: result.get("password").unwrap().clone()
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LoginInfoParseError {
    Success,

    ParsingError,

    Empty
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum LoginResult {
    Success(u128),
    UsernameNoExist,
    PasswordNoExist, // consistency issue

    PasswordWrong,

    UsernameTaken,
}