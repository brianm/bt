use crate::store::{Store, StoreError};
use crate::task::Status;
use colored::*;
use std::path::Path;

pub fn unblock(path: &Path, id: &str, blocker_id: &str) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;
    let mut task = store.load(&task_path)?;

    // Find the blocker to get its full ID
    let blocker_path = store.find(blocker_id)?;
    let blocker = store.load(&blocker_path)?;
    let blocker_full_id = blocker.id().clone();

    // Check if the blocker is actually in blocked_by
    let original_len = task.frontmatter.blocked_by.len();
    task.frontmatter.blocked_by.retain(|b| b != &blocker_full_id);

    if task.frontmatter.blocked_by.len() == original_len {
        return Err(StoreError::Parse(format!(
            "Task {} is not blocked by {}",
            task.id(),
            blocker_full_id
        )));
    }

    // Add log entry
    let author = store.get_author();
    task.add_log(
        &format!("Removed blocker: {}", blocker_full_id),
        author.as_deref(),
    );

    store.save(&task, &task_path)?;

    println!(
        "{} Removed blocker {} from {}",
        "info:".blue(),
        blocker_full_id,
        task.id()
    );

    // If no more blockers and task is in blocked/, move it back to open/
    if task.frontmatter.blocked_by.is_empty() {
        let current_status = store.status_from_path(&task_path);
        if matches!(current_status, Some(Status::Blocked)) {
            let new_path = store.move_to_status(&task_path, Status::Open)?;
            println!(
                "{} All blockers removed, moved to: {}",
                "info:".blue(),
                new_path.display()
            );
        }
    }

    Ok(())
}
