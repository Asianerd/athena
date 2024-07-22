use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::pointer::Pointer;

#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct RawTask {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String
}
impl Into<Task> for RawTask {
    fn into(self) -> Task {
        Task {
            id: self.id,
            project_id: self.project_id,
            title: self.title.clone(),
            description: self.description.clone(),
            parents: vec![],
            children: vec![]
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct Task { // multi-dimensional dynamic doubly linked list
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,

    pub parents: Vec<i64>,
    pub children: Vec<i64>
}
impl Task {
    pub fn create_lookup_table(raw: Vec<RawTask>, pointers: Vec<Pointer>) -> HashMap<i64, Task> {
        let mut result: HashMap<i64, Task> = HashMap::new();
        for t in &raw {
            result.insert(t.id, Task {
                id: t.id,
                project_id: t.project_id,
                title: t.title.clone(),
                description: t.description.clone(),
                parents: Pointer::fetch_parents(t.id, &pointers),
                children: Pointer::fetch_children(t.id, &pointers)
            });
        }

        result
    }
}
