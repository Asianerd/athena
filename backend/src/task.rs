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
            children: vec![],
            branches: vec![]
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
    pub children: Vec<Task>,
    pub branches: Vec<Task>
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
                if i.parent != -1 {
                    // if true, then represents start of a children branch
                    continue;
                }

                parent = Some(Task {
                    id: i.id,
                    project_id: i.project_id,
                    title: i.title.clone(),
                    description: i.description.clone(),
                    origin: i.origin,
                    parent: i.parent,
                    children: vec![],
                    branches: vec![]
                });
            }
        }
        if parent.is_none() {
            return None;
        }
        let mut parent = parent.unwrap();
        // Task::remove_from_collection(parent.id, &mut raw);
        // premature optimization is the root of all evil

        parent.find_all_branches(&raw);
        // parent.find_all_children(collection);

        Some(parent)

    }

    fn find_all_branches(&mut self, collection: &Vec<RawTask>) {
        let mut branches = Task::next_branch(self.id, collection);
        for i in &mut branches {
            i.find_all_branches(collection);
        }
        self.branches = branches;
    }

    fn next_branch(raw: i64, collection: &Vec<RawTask>) -> Vec<Task> {
        collection.iter().filter(|x| x.origin == raw).map(|x| Into::<Task>::into(x.clone())).collect()
    }

    fn next_children(raw: i64, collection: &Vec<RawTask>) -> Vec<Task> {
        collection.iter().filter(|x| x.parent == raw).map(|x| Into::<Task>::into(x.clone())).collect()
    }

    fn find_all_children(&mut self, collection: &Vec<RawTask>) {
        let mut children = Task::next_children(self.id, collection);
        for i in &mut children {
            i.find_all_children(collection);
        }
        self.children = children;
    }
}