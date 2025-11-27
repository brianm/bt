use crate::store::{Store, StoreError};
use crate::task::Status;
use colored::*;
use std::path::Path;

pub fn reopen(path: &Path, id: &str) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;

    // Check if already in an active state
    let current_status = store.status_from_path(&task_path);
    if matches!(
        current_status,
        Some(Status::Open) | Some(Status::InProgress) | Some(Status::Blocked)
    ) {
        println!("{} Task is already open", "warning:".yellow());
        return Ok(());
    }

    let mut task = store.load(&task_path)?;

    let author = store.get_author();
    task.add_log("Reopened.", author.as_deref());

    store.save(&task, &task_path)?;
    let new_path = store.move_to_status(&task_path, Status::Open)?;

    println!("{} Reopened: {}", "info:".blue(), task.id());
    println!("{} Moved to: {}", "info:".blue(), new_path.display());

    Ok(())
}
