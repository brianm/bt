use crate::store::{Store, StoreError};
use crate::task::{Priority, Status};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::path::Path;

/// JSON output structure for a task
#[derive(Serialize)]
struct TaskJson {
    id: String,
    title: String,
    status: Status,
    priority: Priority,
    tags: Vec<String>,
    blocked_by: Vec<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    author: Option<String>,
    body: String,
    log: String,
}

pub fn show(path: &Path, id: &str, json: bool) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;
    let task = store.load(&task_path)?;
    let status = store.status_from_path(&task_path).unwrap_or(Status::Open);

    if json {
        let task_json = TaskJson {
            id: task.id().full().to_string(),
            title: task.title().to_string(),
            status,
            priority: task.priority(),
            tags: task.frontmatter.tags.clone(),
            blocked_by: task.frontmatter.blocked_by.iter().map(|id| id.full().to_string()).collect(),
            created: task.frontmatter.created,
            updated: task.frontmatter.updated,
            author: task.frontmatter.author.clone(),
            body: task.body.clone(),
            log: task.log.clone(),
        };
        println!("{}", serde_json::to_string_pretty(&task_json).unwrap_or_default());
    } else {
        println!("{}", task.to_markdown());
    }

    Ok(())
}
