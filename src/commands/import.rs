use crate::id::TaskId;
use crate::store::{Store, StoreError};
use crate::task::{Priority, Status, Task};
use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Task definition in import file
#[derive(Deserialize)]
struct TaskDef {
    /// Alias for referencing this task in blocked_by
    alias: Option<String>,
    /// Task title (required)
    title: String,
    /// Priority (optional, defaults to medium)
    priority: Option<String>,
    /// Tags (optional)
    #[serde(default)]
    tags: Vec<String>,
    /// Aliases of tasks that block this one
    #[serde(default)]
    blocked_by: Vec<String>,
    /// Task body/description
    body: Option<String>,
}

pub fn import(path: &Path, file: &str) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let author = store.get_author();

    // Read and parse the YAML file
    let content = fs::read_to_string(file)
        .map_err(StoreError::Io)?;

    let task_defs: Vec<TaskDef> = serde_yaml::from_str(&content)
        .map_err(|e| StoreError::Parse(format!("Failed to parse YAML: {}", e)))?;

    // Map from alias to created task ID
    let mut alias_to_id: HashMap<String, TaskId> = HashMap::new();

    // First pass: create all tasks without dependencies
    let mut tasks_to_create: Vec<(Task, Vec<String>)> = Vec::new();

    for def in task_defs {
        let mut task = Task::new(&def.title, author.clone());

        // Set priority
        if let Some(p) = &def.priority {
            if let Ok(priority) = p.parse::<Priority>() {
                task.frontmatter.priority = priority;
            }
        }

        // Set tags
        task.frontmatter.tags = def.tags;

        // Set body
        if let Some(body) = def.body {
            task.body = body;
        }

        // Store the alias mapping
        if let Some(alias) = &def.alias {
            alias_to_id.insert(alias.clone(), task.id().clone());
        }

        tasks_to_create.push((task, def.blocked_by));
    }

    // Second pass: resolve blocked_by aliases and create tasks
    for (mut task, blocked_by_aliases) in tasks_to_create {
        let mut has_unresolved_blockers = false;

        for alias in blocked_by_aliases {
            // Try to resolve as alias first
            if let Some(blocker_id) = alias_to_id.get(&alias) {
                task.frontmatter.blocked_by.push(blocker_id.clone());
                has_unresolved_blockers = true;
            } else {
                // Try to find as existing task ID
                match store.find(&alias) {
                    Ok(blocker_path) => {
                        let blocker = store.load(&blocker_path)?;
                        task.frontmatter.blocked_by.push(blocker.id().clone());

                        // Check if blocker is unresolved
                        if let Some(status) = store.status_from_path(&blocker_path) {
                            if !matches!(status, Status::Closed | Status::Cancelled) {
                                has_unresolved_blockers = true;
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("{} Unknown blocker alias or ID: {}", "warning:".yellow(), alias);
                    }
                }
            }
        }

        // Create the task
        let mut task_path = store.create(&task)?;

        // Move to blocked if needed
        if has_unresolved_blockers {
            task_path = store.move_to_status(&task_path, Status::Blocked)?;
        }

        println!("{}", task.id());
        println!(
            "{} Created: {}",
            "info:".blue(),
            task_path.display()
        );
    }

    Ok(())
}
