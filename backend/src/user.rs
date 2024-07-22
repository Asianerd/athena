use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{login_info::{LoginInformation, LoginResult}, utils};

#[derive(FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String
}
impl User {
    pub async fn lookup_user_id(id: &i64, db: &Pool<Sqlite>) -> i64 {
        let result: (i64,) = sqlx::query_as("select count(*) from user where id = $1;")
            .bind(id)
            .fetch_one(db)
            .await
            .unwrap();
        result.0
    }
}



#[post("/", data="<login>")]
pub async fn test(db: &State<Pool<Sqlite>>, login: LoginInformation) -> String {
    let result = login.login(db.inner()).await;
    match result {
        LoginResult::Success(user_id) => {
            utils::parse_response(Ok(user_id))
        },
        _ => utils::parse_response(Err(result))
    }
}
