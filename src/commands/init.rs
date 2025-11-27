use crate::store::{Store, StoreError};
use colored::*;
use std::path::Path;

pub fn init(path: &Path) -> Result<(), StoreError> {
    match Store::init(path) {
        Ok(store) => {
            println!(
                "{} Initialized task tracker in {}",
                "info:".blue(),
                store.tasks_dir().display()
            );
            println!("  {}/open/", store.tasks_dir().display());
            println!("  {}/closed/", store.tasks_dir().display());
            println!("  {}/config.yaml", store.tasks_dir().display());
            Ok(())
        }
        Err(StoreError::AlreadyInitialized) => {
            println!("{} Task directory already exists", "warning:".yellow());
            Ok(())
        }
        Err(e) => Err(e),
    }
}
