use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{login_info::{self, LoginInformation, LoginResult}, project_members::ProjectMembers, task::{RawTask, Task}, utils};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub title: String
}
impl Project {
    pub async fn fetch_all(db: &Pool<Sqlite>) -> Vec<Project> {
        sqlx::query_as("select * from project;")
            .fetch_all(db)
            .await
            .unwrap()
    }

    pub async fn fetch_tasks(db: &Pool<Sqlite>, project_id: i64) -> Option<Task> {
        Task::create_task(
            sqlx::query_as("select * from task where project_id = $1;")
                .bind(project_id)
                .fetch_all(db)
                .await
                .unwrap()
        )
    }
}

#[get("/")]
pub async fn fetch_all(db: &State<Pool<Sqlite>>) -> String {
    utils::parse_response(Ok(Project::fetch_all(db.inner()).await))
}

#[post("/", data="<login>")]
pub async fn fetch_own_projects(db: &State<Pool<Sqlite>>, login: LoginInformation) -> String {
    let result = login.login(db.inner()).await;
    match result {
        LoginResult::Success(user_id) => {
            utils::parse_response(Ok(ProjectMembers::fetch_projects(db, user_id).await))
        },
        _ => utils::parse_response(Err(result))
    }
}

// #[post("/", data="<login>")]
#[get("/<project_id>")]
pub async fn fetch_project_members(db: &State<Pool<Sqlite>>, project_id: i64) -> String {
    utils::parse_response(Ok(ProjectMembers::fetch_members(db.inner(), project_id).await))
}



#[get("/<project_id>")]
pub async fn fetch_tasks(db: &State<Pool<Sqlite>>, project_id: i64) -> String {
    utils::parse_response(Ok(Project::fetch_tasks(db, project_id).await))
}
