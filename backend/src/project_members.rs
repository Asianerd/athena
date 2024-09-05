use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{login_info::{LoginInformation, LoginResult}, project::Project, user::User, utils::{self, ValueInt}};

#[derive(FromRow, Serialize, Deserialize)]
pub struct ProjectMembers {
    id: i64,
    pub project_id: i64,
    pub user_id: i64
}
impl ProjectMembers {
    pub async fn fetch_projects(db: &Pool<Sqlite>, user_id: i64) -> Vec<Project> {
        sqlx::query_as("select project.* from project, project_members where (project.id = project_members.project_id) and (project_members.user_id = $1);")
            .bind(user_id)
            .fetch_all(db)
            .await
            .unwrap()
    }

    pub async fn fetch_members(db: &Pool<Sqlite>, project_id: i64) -> Vec<User> {
        sqlx::query_as("select user.* from user, project_members where (user.id = project_members.user_id) and (project_members.project_id = $1);")
            .bind(project_id)
            .fetch_all(db)
            .await
            .unwrap()
    }

    pub async fn is_member(db: &Pool<Sqlite>, user_id: i64, project_id: i64) -> bool {
        sqlx::query_as::<_, ValueInt>("select count(*) from project_members where (user_id = $1) and (project_id = $2);")
            .bind(user_id)
            .bind(project_id)
            .fetch_one(db)
            .await
            .unwrap().0 >= 1
    }

    pub async fn add(db: &Pool<Sqlite>, user_id: i64, project_id: i64) {
        sqlx::query("insert into project_members(project_id, user_id) values($1, $2);")
            .bind(project_id)
            .bind(user_id)
            .execute(db)
            .await
            .unwrap();
    }

    pub async fn remove(db: &Pool<Sqlite>, user_id: i64, project_id: i64) {
        sqlx::query("delete from project_members where user_id = $1, project_id = $2;")
            .bind(user_id)
            .bind(project_id)
            .execute(db)
            .await
            .unwrap();
    }
}

#[post("/<project_id>/<user_id>", data="<login>")]
pub async fn add(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64, user_id: i64) -> String {
    let db = db.inner();
    let result = login.login(&db).await;
    match result {
        LoginResult::Success(u) => {
            if ProjectMembers::is_member(db, u, project_id).await {
                ProjectMembers::add(db, user_id, project_id).await;

                return utils::parse_response(Ok("added"));
            }
            utils::parse_response(Ok("not a member of project"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}

#[post("/<project_id>/<user_id>", data="<login>")]
pub async fn remove(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64, user_id: i64) -> String {
    let db = db.inner();
    let result = login.login(&db).await;
    match result {
        LoginResult::Success(u) => {
            if ProjectMembers::is_member(db, u, project_id).await {
                ProjectMembers::remove(db, user_id, project_id).await;

                return utils::parse_response(Ok("removed"));
            }
            utils::parse_response(Ok("not a member of project"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}
