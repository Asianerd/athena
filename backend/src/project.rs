use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{login_info::{LoginInformation, LoginResult}, project_members::ProjectMembers, utils};

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

    pub async fn create(db: &Pool<Sqlite>, title: String, description: String) {
        sqlx::query("insert into project(title, description) values($1, $2);")
            .bind(title)
            .bind(description)
            .execute(db)
            .await
            .unwrap();
    }

    pub async fn delete(db: &Pool<Sqlite>, project_id: i64) {
        sqlx::query("delete from project where id = $1;")
            .bind(project_id)
            .execute(db)
            .await
            .unwrap();
    }

    pub async fn edit(db: &Pool<Sqlite>, project_id: i64, title: String, description: String) {
        sqlx::query("update project set title = $1, description = $2 where id = $3")
            .bind(title)
            .bind(description)
            .bind(project_id)
            .execute(db)
            .await
            .unwrap();
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

#[post("/<project_id>", data="<login>")]
pub async fn fetch_project_members(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64) -> String {
    let db = db.inner();
    let result = login.login(&db).await;
    match result {
        LoginResult::Success(u) => {
            if ProjectMembers::is_member(db, u, project_id).await {
                return utils::parse_response(Ok(ProjectMembers::fetch_members(db, project_id).await));
            }
            utils::parse_response(Ok("not a member of project"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}

#[post("/<title>/<description>", data="<login>")]
pub async fn create(db: &State<Pool<Sqlite>>, login: LoginInformation, title: String, description: String) -> String {
    let db = db.inner();
    let result = login.login(&db).await;
    match result {
        LoginResult::Success(_) => {
            Project::create(db, title, description).await;

            utils::parse_response(Ok("created"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}

#[post("/<project_id>", data="<login>")]
pub async fn delete(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64) -> String {
    let db = db.inner();
    let result = login.login(&db).await;
    match result {
        LoginResult::Success(u) => {
            if ProjectMembers::is_member(db, u, project_id).await {
                Project::delete(db, project_id).await;

                return utils::parse_response(Ok("deleted"));
            }
            utils::parse_response(Ok("not a member of project"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}

#[post("/<project_id>/<title>/<description>", data="<login>")]
pub async fn edit(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64, title: String, description: String) -> String {
    let db = db.inner();
    let result = login.login(&db).await;
    match result {
        LoginResult::Success(u) => {
            if ProjectMembers::is_member(db, u, project_id).await {
                Project::edit(db, project_id, title, description).await;
                return utils::parse_response(Ok("edited"));
            }
            utils::parse_response(Ok("not a member of project"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}
