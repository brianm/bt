use crate::store::{Store, StoreError};
use colored::*;
use std::path::Path;

pub fn log(path: &Path, id: &str, message: &str) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;

    let mut task = store.load(&task_path)?;
    let author = store.get_author();
    task.add_log(message, author.as_deref());

    store.save(&task, &task_path)?;

    println!("{} Added log entry to: {}", "info:".blue(), task.id());

    Ok(())
}
