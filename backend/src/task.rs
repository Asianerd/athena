use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct RawTask { // this is basically a linked list
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub origin: i64, // the task before this
    pub parent: i64 // if isnt -1, then its a subtask in a bigger task
}
impl Into<Task> for RawTask {
    fn into(self) -> Task {
        Task {
            id: self.id,
            project_id: self.project_id,
            title: self.title.clone(),
            description: self.description.clone(),
            origin: self.origin,
            parent: self.parent,
            children: vec![]
        }
    }
}

#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct Task { // multi-dimensional dynamic linked list (or something like that idk)
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub origin: i64, // the task before this, -1 if starting task
    pub parent: i64, // if isnt -1, then its a subtask in a bigger task
    pub children: Vec<Task>
}
impl Task {
    pub fn create_task(raw: Vec<RawTask>) -> Option<Task> {
        if raw.is_empty() {
            return None;
        }
        let raw = raw.clone();

        let mut parent: Option<Task> = None;
        for i in &raw {
            if i.origin == -1 {
                parent = Some(Task {
                    id: i.id,
                    project_id: i.project_id,
                    title: i.title.clone(),
                    description: i.description.clone(),
                    origin: i.origin,
                    parent: i.parent,
                    children: vec![]
                });
            }
        }
        if parent.is_none() {
            return None;
        }
        let mut parent = parent.unwrap();
        // Task::remove_from_collection(parent.id, &mut raw);
        // premature optimization is the root of all evil

        parent.find_all_children(&raw);

        Some(parent)

    }

    fn find_all_children(&mut self, collection: &Vec<RawTask>) {
        let mut children = Task::next(self.id, collection);
        for i in &mut children {
            i.find_all_children(collection);
        }
        self.children = children;
    }

    fn next(raw: i64, collection: &Vec<RawTask>) -> Vec<Task> {
        collection.iter().filter(|x| x.origin == raw).map(|x| Into::<Task>::into(x.clone())).collect()
    }

    fn remove_from_collection(raw: i64, collection: &mut Vec<RawTask>) {
        let mut target: Option<usize> = None;
        for (index, i) in collection.iter().enumerate() {
            if i.id == raw {
                target = Some(index);
            }
        }
        if target.is_some() {
            collection.remove(target.unwrap());
        }
    }
}