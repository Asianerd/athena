use std::collections::HashMap;

use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{login_info::{LoginInformation, LoginResult}, project_members::ProjectMembers, utils};


#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct Task { // multi-dimensional linked list
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub parent: i64,
}
impl Task {
    pub fn into_hashmap(collection: Vec<Task>) -> HashMap<i64, Task> {
        collection
            .iter()
            .map(|t| (t.id, t.clone()))
            .collect::<HashMap<i64, Task>>()
    }

    pub async fn fetch_tasks(db: &Pool<Sqlite>, project_id: i64) -> HashMap<i64, Task> {
        Task::into_hashmap(
            sqlx::query_as("select * from task where project_id = $1;")
                .bind(project_id)
                .fetch_all(db)
                .await
                .unwrap()
        )
    }

    pub async fn create_task(db: &Pool<Sqlite>, project_id: i64, title: String, description: String, parent: i64, author: i64) {
        sqlx::query("insert into task(project_id, title, description, parent, author) values($1, $2, $3, $4, $5);")
            .bind(project_id)
            .bind(title)
            .bind(description)
            .bind(parent)
            .bind(author)
            .execute(db)
            .await
            .unwrap();
    }
    
    pub async fn edit_task(db: &Pool<Sqlite>, task_id: i64, title: String, description: String, parent: i64) {
        sqlx::query("update task set title = $1, description = $2, parent = $3 where id = $4;")
            .bind(title)
            .bind(description)
            .bind(parent)
            .bind(task_id)
            .execute(db)
            .await
            .unwrap();
    }

    pub async fn delete_task(db: &Pool<Sqlite>, task_id: i64) {
        sqlx::query("delete from task where id = $1")
            .bind(task_id)
            .execute(db)
            .await
            .unwrap();
    }
}

#[post("/<project_id>", data="<login>")]
pub async fn fetch_tasks(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64) -> String {
    let db = db.inner();
    let result = login.login(db).await;
    match result {
        LoginResult::Success(u) => {
            if ProjectMembers::is_member(&db, u, project_id).await {
                return utils::parse_response(Ok(Task::fetch_tasks(db, project_id).await));
            }
            utils::parse_response(Ok("not a member of project"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}

// CREATE TABLE task(id int primary key, project_id int, title varchar, description varchar);
#[post("/<project_id>/<title>/<description>/<parent>", data="<login>")]
pub async fn create(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64, title: String, description: String, parent: i64) -> String {
    let db = db.inner();
    let result = login.login(&db).await;
    match result {
        LoginResult::Success(u) => {
            Task::create_task(db, project_id, urlencoding::decode(&title).unwrap().to_string(), urlencoding::decode(&description).unwrap().to_string(), parent, u).await;

            utils::parse_response(Ok("created"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}

#[post("/<project_id>/<task_id>", data="<login>")]
pub async fn delete(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64, task_id: i64) -> String {
    let db = db.inner();
    let result = login.login(db).await;
    match result {
        LoginResult::Success(u) => {
            if ProjectMembers::is_member(db, u, project_id).await {
                Task::delete_task(db, task_id).await;
                return utils::parse_response(Ok("deleted"));
            }

            utils::parse_response(Ok("not a member of project"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}

#[post("/<project_id>/<task_id>/<title>/<description>/<parent>", data="<login>")]
pub async fn edit(db: &State<Pool<Sqlite>>, login: LoginInformation, project_id: i64, task_id: i64, title: String, description: String, parent: i64) -> String {
    let db = db.inner();
    let result = login.login(&db).await;
    match result {
        LoginResult::Success(u) => {
            if ProjectMembers::is_member(db, u, project_id).await {
                Task::edit_task(db, task_id, urlencoding::decode(&title).unwrap().to_string(), urlencoding::decode(&description).unwrap().to_string(), parent).await;
                return utils::parse_response(Ok("edited"));
            }

            utils::parse_response(Ok("not a member of project"))
        },
        _ => {
            utils::parse_response(Err(result))
        }
    }
}