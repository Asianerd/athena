use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Execute, Pool, Sqlite};
use strum_macros::EnumString;

use crate::utils;

#[derive(FromRow)]
pub struct Pointer {
    pub id: i64,
    pub project_id: i64,
    pub parent: i64,
    pub child: i64
}
impl Pointer {
    pub async fn fetch_all(db: &Pool<Sqlite>, project_id: i64) -> Vec<Pointer> {
        sqlx::query_as("select * from pointer where project_id = $1")
            .bind(project_id)
            .fetch_all(db)
            .await
            .unwrap()
    }

    pub fn fetch_children(task_id: i64, collection: &Vec<Pointer>) -> Vec<i64> {
        collection.iter().filter(|x| x.parent == task_id).map(|x| x.child).collect()
    }

    pub fn fetch_parents(task_id: i64, collection: &Vec<Pointer>) -> Vec<i64> {
        collection.iter().filter(|x| x.child == task_id).map(|x| x.parent).collect()
    }

    pub async fn create_pointer(db: &Pool<Sqlite>, project_id: i64, parent: i64, child: i64) -> PointerResult {
        if child == parent {
            return PointerResult::ToSelf;
        }

        if sqlx::query_as::<_, utils::ValueInt>("select count(id) from pointer where parent = $1 and child = $2;")
            .bind(parent)
            .bind(child)
            .fetch_one(db)
            .await
            .unwrap().0 >= 1 {
            return PointerResult::AlreadyExist;
        }

        sqlx::query("insert into pointer(project_id, parent, child) values($1, $2, $3);")
            .bind(project_id)
            .bind(parent)
            .bind(child)
            .execute(db)
            .await
            .unwrap();

        PointerResult::Success
    }

    pub async fn delete_pointer(db: &Pool<Sqlite>, project_id: i64, parent: i64, child: i64) -> PointerResult {
        if sqlx::query_as::<_, utils::ValueInt>("select count(id) from pointer where parent = $1 and child = $2;")
            .bind(parent)
            .bind(child)
            .fetch_one(db)
            .await
            .unwrap().0 <= 0 {
            return PointerResult::DoesntExist;
        }

        sqlx::query("delete from pointer where project_id = $1 and parent = $2 and child = $3;")
            .bind(project_id)
            .bind(parent)
            .bind(child)
            .execute(db)
            .await
            .unwrap();

        PointerResult::Success
    }
}

// #[derive()]
#[derive(EnumString, Serialize, Deserialize)]
pub enum PointerResult {
    Success,
    DoesntExist,
    AlreadyExist,
    ToSelf
}

#[get("/<project_id>/<parent>/<child>")]
pub async fn create_pointer(db: &State<Pool<Sqlite>>, project_id: i64, parent: i64, child: i64) -> String {
    utils::parse_response(Ok(Pointer::create_pointer(db.inner(), project_id, parent, child).await))
}

#[get("/<project_id>/<parent>/<child>")]
pub async fn delete_pointer(db: &State<Pool<Sqlite>>, project_id: i64, parent: i64, child: i64) -> String {
    utils::parse_response(Ok(Pointer::delete_pointer(db.inner(), project_id, parent, child).await))
}
