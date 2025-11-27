use crate::store::{Store, StoreError};
use crate::task::Status;
use colored::*;
use std::path::Path;

pub fn close(path: &Path, id: &str, reason: Option<&str>) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;

    // Check if already in a terminal state
    let current_status = store.status_from_path(&task_path);
    if matches!(current_status, Some(Status::Closed) | Some(Status::Cancelled)) {
        println!("{} Task is already closed", "warning:".yellow());
        return Ok(());
    }

    let mut task = store.load(&task_path)?;
    let task_id = task.id().clone();

    let message = match reason {
        Some(r) => format!("Closed: {}", r),
        None => "Closed.".to_string(),
    };

    let author = store.get_author();
    task.add_log(&message, author.as_deref());

    store.save(&task, &task_path)?;
    let new_path = store.move_to_status(&task_path, Status::Closed)?;

    println!("{} Closed: {}", "info:".blue(), task.id());
    println!("{} Moved to: {}", "info:".blue(), new_path.display());

    // Unblock any tasks that were waiting on this one
    let unblocked = store.unblock_waiting_tasks(&task_id)?;
    for unblocked_path in unblocked {
        let unblocked_task = store.load(&unblocked_path)?;
        println!(
            "{} Unblocked: {} ({})",
            "info:".blue(),
            unblocked_task.id(),
            unblocked_task.title()
        );
    }

    Ok(())
}
