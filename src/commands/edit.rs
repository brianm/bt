use crate::store::{Store, StoreError};
use chrono::Utc;
use std::path::Path;
use std::process::Command;

pub fn edit(path: &Path, id: &str) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    let status = Command::new(&editor)
        .arg(&task_path)
        .status()
        .map_err(|e| StoreError::Parse(format!("Failed to launch editor: {}", e)))?;

    if !status.success() {
        return Err(StoreError::Parse("Editor exited with error".to_string()));
    }

    // Update the 'updated' timestamp
    let mut task = store.load(&task_path)?;
    task.frontmatter.updated = Utc::now();
    store.save(&task, &task_path)?;

    Ok(())
}
