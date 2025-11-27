use crate::id::TaskId;
use crate::store::{Store, StoreError};
use crate::task::Priority;
use colored::*;
use std::path::Path;

pub fn ready(path: &Path) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let tasks = store.list_ready()?;

    // Collect all IDs for computing shortest unique prefixes
    let all_ids: Vec<&TaskId> = tasks.iter().map(|(_, task)| task.id()).collect();

    for (_, task) in &tasks {
        let short_id = task.id().shortest_unique_prefix(&all_ids);

        let priority_colored = match task.priority() {
            Priority::Critical => "critical".red(),
            Priority::High => "high".yellow(),
            Priority::Medium => "medium".normal(),
            Priority::Low => "low".blue(),
        };

        println!("{}\t{}\t{}", short_id, priority_colored, task.title());
    }

    Ok(())
}
