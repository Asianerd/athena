use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{project::Project, user::User};

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
}
