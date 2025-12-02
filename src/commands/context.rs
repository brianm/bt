use crate::id::TaskId;
use crate::store::{Store, StoreError};
use crate::task::Status;
use colored::*;
use std::path::Path;

pub fn context(path: &Path, id: &str) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let task_path = store.find(id)?;
    let task = store.load(&task_path)?;
    let status = store.status_from_path(&task_path).unwrap_or(Status::Open);

    // Get all tasks for ID prefix computation
    let all_tasks = store.list_all()?;
    let all_ids: Vec<&TaskId> = all_tasks.iter().map(|(_, t)| t.id()).collect();
    let short_id = task.id().shortest_unique_prefix(&all_ids);

    // === Main Task ===
    println!("{}", "=== Task ===".bold());
    println!();
    println!("{} {}", "ID:".dimmed(), short_id);
    println!("{} {}", "Title:".dimmed(), task.title().bold());
    println!("{} {}", "Status:".dimmed(), format_status(status));
    println!("{} {}", "Priority:".dimmed(), task.priority());
    if !task.frontmatter.tags.is_empty() {
        println!("{} {}", "Tags:".dimmed(), task.frontmatter.tags.join(", "));
    }
    println!();

    // Body (without log section)
    let body_without_log: String = task
        .body
        .lines()
        .take_while(|line| !line.starts_with("## Log"))
        .collect::<Vec<_>>()
        .join("\n");
    if !body_without_log.trim().is_empty() {
        println!("{}", body_without_log.trim());
        println!();
    }

    // === Blocking Tasks (tasks that block this one) ===
    if !task.frontmatter.blocked_by.is_empty() {
        println!("{}", "=== Blocked By ===".bold());
        println!();
        for blocker_id in &task.frontmatter.blocked_by {
            let blocker_short = blocker_id.shortest_unique_prefix(&all_ids);
            match store.find(&blocker_id.full()) {
                Ok(blocker_path) => {
                    let blocker = store.load(&blocker_path)?;
                    let blocker_status = store.status_from_path(&blocker_path).unwrap_or(Status::Open);
                    println!(
                        "  {} {} [{}]",
                        blocker_short.cyan(),
                        blocker.title(),
                        format_status(blocker_status)
                    );
                }
                Err(_) => {
                    println!("  {} {}", blocker_short.cyan(), "(not found)".dimmed());
                }
            }
        }
        println!();
    }

    // === Tasks this blocks ===
    let blocks: Vec<_> = all_tasks
        .iter()
        .filter(|(_, t)| t.frontmatter.blocked_by.contains(task.id()))
        .collect();

    if !blocks.is_empty() {
        println!("{}", "=== Blocks ===".bold());
        println!();
        for (blocked_path, blocked_task) in blocks {
            let blocked_short = blocked_task.id().shortest_unique_prefix(&all_ids);
            let blocked_status = store.status_from_path(blocked_path).unwrap_or(Status::Open);
            println!(
                "  {} {} [{}]",
                blocked_short.cyan(),
                blocked_task.title(),
                format_status(blocked_status)
            );
        }
        println!();
    }

    // === Recent Log Entries ===
    // Use the task's log field directly
    if !task.log.trim().is_empty() {
        let entries: Vec<_> = task
            .log
            .split("\n---\n# Log: ")
            .take(5) // Last 5 entries
            .collect();

        if !entries.is_empty() {
            println!("{}", "=== Recent Activity ===".bold());
            println!();
            for entry in entries {
                let lines: Vec<&str> = entry.lines().collect();
                if lines.is_empty() {
                    continue;
                }

                // Parse header: "2025-11-26T23:42:29Z Author Name"
                // Handle first entry which starts with "---\n# Log: " (no leading newline)
                let header = lines[0]
                    .trim_start_matches("---")
                    .trim_start()
                    .trim_start_matches("# Log: ");
                let parts: Vec<&str> = header.splitn(2, ' ').collect();
                if parts.len() >= 2 {
                    let timestamp = parts[0];
                    let author = parts[1];
                    // Format timestamp nicely
                    let formatted_time = timestamp
                        .parse::<chrono::DateTime<chrono::Utc>>()
                        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                        .unwrap_or_else(|_| timestamp.to_string());

                    println!("  {} {}", formatted_time.dimmed(), author.dimmed());
                }

                // Get the message (first non-empty line after header)
                let message: String = lines[1..]
                    .iter()
                    .skip_while(|l| l.is_empty())
                    .take(2) // First 2 lines of message
                    .map(|s| format!("    {}", s))
                    .collect::<Vec<_>>()
                    .join("\n");

                if !message.trim().is_empty() {
                    println!("{}", message);
                }
                println!();
            }
        }
    }

    Ok(())
}

fn format_status(status: Status) -> colored::ColoredString {
    match status {
        Status::Open => "open".green(),
        Status::InProgress => "in-progress".yellow(),
        Status::Blocked => "blocked".red(),
        Status::Closed => "closed".blue(),
        Status::Cancelled => "cancelled".red(),
    }
}
