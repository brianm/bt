use crate::prefix::PrefixResolver;
use crate::store::{Store, StoreError};
use crate::task::Priority;
use crate::term::LineFormatter;
use colored::*;
use std::path::Path;

pub fn ready(path: &Path) -> Result<(), StoreError> {
    let store = Store::open(path)?;
    let tasks = store.list_ready()?;

    // Resolve shortest unique prefixes across ALL tasks (including closed/cancelled)
    // This ensures displayed prefixes work with `yatl edit`, which searches all directories
    let resolver = PrefixResolver::new(&store)?;

    // Auto-detect terminal width for line truncation
    let formatter = LineFormatter::auto();

    // Fixed columns: short_id (8) + tab + priority (8) + tab = ~18 chars
    const FIXED_COLS: usize = 18;

    for (_, task) in &tasks {
        let short_id = resolver.shortest_prefix(task.id());

        let priority_colored = match task.priority() {
            Priority::Critical => "critical".red(),
            Priority::High => "high".yellow(),
            Priority::Medium => "medium".normal(),
            Priority::Low => "low".blue(),
        };

        let title = formatter.truncate(task.title(), FIXED_COLS);
        println!("{}\t{}\t{}", short_id, priority_colored, title);
    }

    Ok(())
}
