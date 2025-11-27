use crate::store::{Store, StoreError};
use crate::task::Status;
use colored::*;
use std::path::Path;

pub fn block(path: &Path, task_id: &str, blocker_id: &str) -> Result<(), StoreError> {
    let store = Store::open(path)?;

    let task_path = store.find(task_id)?;
    let blocker_path = store.find(blocker_id)?;

    let mut task = store.load(&task_path)?;
    let blocker = store.load(&blocker_path)?;

    // Check if blocker is already resolved
    let blocker_status = store.status_from_path(&blocker_path);
    let blocker_resolved =
        matches!(blocker_status, Some(Status::Closed) | Some(Status::Cancelled));

    // Add to blocked_by list
    task.frontmatter.blocked_by.push(blocker.id().clone());

    let author = store.get_author();
    task.add_log(
        &format!("Added blocker: {}", blocker.id()),
        author.as_deref(),
    );

    store.save(&task, &task_path)?;

    println!(
        "{} Task {} is now blocked by {}",
        "info:".blue(),
        task.id(),
        blocker.id()
    );

    // If the blocker is not resolved, move task to blocked/
    if !blocker_resolved {
        let current_status = store.status_from_path(&task_path);
        if !matches!(current_status, Some(Status::Blocked)) {
            let new_path = store.move_to_status(&task_path, Status::Blocked)?;
            println!("{} Moved to: {}", "info:".blue(), new_path.display());
        }
    }

    Ok(())
}
