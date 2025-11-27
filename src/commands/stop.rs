use crate::store::{Store, StoreError};
use crate::task::Status;
use colored::*;
use std::path::Path;

pub fn stop(path: &Path, id: &str) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;

    let current_status = store.status_from_path(&task_path);

    // Can only stop tasks that are in-progress
    if !matches!(current_status, Some(Status::InProgress)) {
        let status_str = current_status
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        return Err(StoreError::Parse(format!(
            "Cannot stop task with status '{}'. Only 'in-progress' tasks can be stopped.",
            status_str
        )));
    }

    let mut task = store.load(&task_path)?;

    let author = store.get_author();
    task.add_log("Stopped working.", author.as_deref());

    store.save(&task, &task_path)?;
    let new_path = store.move_to_status(&task_path, Status::Open)?;

    println!("{} Stopped: {}", "info:".blue(), task.id());
    println!("{} Moved to: {}", "info:".blue(), new_path.display());

    Ok(())
}
