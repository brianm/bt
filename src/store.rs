use crate::config::Config;
use crate::id::TaskId;
use crate::task::{Status, Task};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

const TASKS_DIR: &str = ".tasks";

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Not a bt-enabled directory. Run 'bt init' first.")]
    NotInitialized,

    #[error("Task directory already exists")]
    AlreadyInitialized,

    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Ambiguous ID '{0}' matches multiple tasks")]
    AmbiguousId(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, StoreError>;

/// File-based task store with directory-based status
pub struct Store {
    tasks_dir: PathBuf,
    config: Config,
}

impl Store {
    /// Get the directory for a given status
    fn status_dir(&self, status: Status) -> PathBuf {
        let dir_name = match status {
            Status::Open => "open",
            Status::InProgress => "in-progress",
            Status::Blocked => "blocked",
            Status::Closed => "closed",
            Status::Cancelled => "cancelled",
        };
        self.tasks_dir.join(dir_name)
    }

    /// Get all status directories
    fn all_status_dirs(&self) -> Vec<PathBuf> {
        vec![
            self.status_dir(Status::Open),
            self.status_dir(Status::InProgress),
            self.status_dir(Status::Blocked),
            self.status_dir(Status::Closed),
            self.status_dir(Status::Cancelled),
        ]
    }

    /// Derive status from a file path
    pub fn status_from_path(&self, path: &Path) -> Option<Status> {
        let parent = path.parent()?;
        let dir_name = parent.file_name()?.to_str()?;
        match dir_name {
            "open" => Some(Status::Open),
            "in-progress" => Some(Status::InProgress),
            "blocked" => Some(Status::Blocked),
            "closed" => Some(Status::Closed),
            "cancelled" => Some(Status::Cancelled),
            _ => None,
        }
    }

    /// Open an existing store
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let tasks_dir = root.join(TASKS_DIR);

        if !tasks_dir.exists() {
            return Err(StoreError::NotInitialized);
        }

        let config = Config::load(&tasks_dir);

        Ok(Store { tasks_dir, config })
    }

    /// Initialize a new store
    pub fn init(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let tasks_dir = root.join(TASKS_DIR);

        if tasks_dir.exists() {
            return Err(StoreError::AlreadyInitialized);
        }

        let store = Store {
            tasks_dir: tasks_dir.clone(),
            config: Config::default(),
        };

        // Create all status directories
        for dir in store.all_status_dirs() {
            fs::create_dir_all(&dir)?;
            fs::write(dir.join(".gitkeep"), "")?;
        }

        // Create .gitattributes for better merging
        fs::write(
            tasks_dir.join(".gitattributes"),
            "# Use union merge for task files - concatenates both sides\n\
             # This helps with the append-only log section\n\
             *.md merge=union\n",
        )?;

        // Create default config
        let config = Config {
            default_author: Config::default().get_author(),
        };
        config.save(&tasks_dir)?;

        Ok(Store { tasks_dir, config })
    }

    /// Get the default author
    pub fn get_author(&self) -> Option<String> {
        self.config.get_author()
    }

    /// Create a new task (in open/ directory)
    pub fn create(&self, task: &Task) -> Result<PathBuf> {
        let filename = format!("{}.md", task.id());
        let path = self.status_dir(Status::Open).join(&filename);

        let content = task.to_markdown();
        fs::write(&path, content)?;

        Ok(path)
    }

    /// Find a task by ID or prefix across all directories
    pub fn find(&self, id_or_prefix: &str) -> Result<PathBuf> {
        let mut matches = vec![];

        for dir in self.all_status_dirs() {
            if !dir.exists() {
                continue;
            }

            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().is_some_and(|e| e == "md") {
                    let filename = path.file_stem().unwrap_or_default().to_string_lossy();

                    // Check exact match
                    if filename == id_or_prefix {
                        return Ok(path);
                    }

                    // Check prefix match
                    let id = TaskId::from_string(filename.to_string());
                    if id.matches_prefix(id_or_prefix) {
                        matches.push(path);
                    }
                }
            }
        }

        match matches.len() {
            0 => Err(StoreError::TaskNotFound(id_or_prefix.to_string())),
            1 => Ok(matches.remove(0)),
            _ => Err(StoreError::AmbiguousId(id_or_prefix.to_string())),
        }
    }

    /// Load a task from a path
    pub fn load(&self, path: &Path) -> Result<Task> {
        let content = fs::read_to_string(path)?;
        Task::parse(&content).map_err(StoreError::Parse)
    }

    /// Save a task back to its file
    pub fn save(&self, task: &Task, path: &Path) -> Result<()> {
        let content = task.to_markdown();
        fs::write(path, content)?;
        Ok(())
    }

    /// List all tasks in a directory
    fn list_dir(&self, dir: &Path) -> Result<Vec<(PathBuf, Task)>> {
        let mut tasks = vec![];

        if !dir.exists() {
            return Ok(tasks);
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|e| e == "md") {
                if let Ok(task) = self.load(&path) {
                    tasks.push((path, task));
                }
            }
        }

        Ok(tasks)
    }

    /// List tasks by status
    pub fn list_by_status(&self, status: Status) -> Result<Vec<(PathBuf, Task)>> {
        self.list_dir(&self.status_dir(status))
    }

    /// List all tasks across all directories
    pub fn list_all(&self) -> Result<Vec<(PathBuf, Task)>> {
        let mut tasks = vec![];
        for dir in self.all_status_dirs() {
            tasks.extend(self.list_dir(&dir)?);
        }
        Ok(tasks)
    }

    /// List all non-terminal tasks (open, in-progress, blocked)
    pub fn list_active(&self) -> Result<Vec<(PathBuf, Task)>> {
        let mut tasks = vec![];
        tasks.extend(self.list_by_status(Status::Open)?);
        tasks.extend(self.list_by_status(Status::InProgress)?);
        tasks.extend(self.list_by_status(Status::Blocked)?);
        Ok(tasks)
    }

    /// Move a task to a new status directory
    pub fn move_to_status(&self, path: &Path, status: Status) -> Result<PathBuf> {
        let filename = path
            .file_name()
            .ok_or_else(|| StoreError::Parse("Invalid path".to_string()))?;
        let new_path = self.status_dir(status).join(filename);
        fs::rename(path, &new_path)?;
        Ok(new_path)
    }

    /// Get ready tasks (in open/ directory with no unresolved blockers)
    pub fn list_ready(&self) -> Result<Vec<(PathBuf, Task)>> {
        let open_tasks = self.list_by_status(Status::Open)?;
        let all_tasks = self.list_all()?;

        let ready: Vec<(PathBuf, Task)> = open_tasks
            .into_iter()
            .filter(|(_, task)| {
                // Check if all blockers are resolved (in closed or cancelled)
                task.frontmatter.blocked_by.iter().all(|blocker_id| {
                    all_tasks.iter().any(|(blocker_path, t)| {
                        t.id() == blocker_id && {
                            let status = self.status_from_path(blocker_path);
                            matches!(status, Some(Status::Closed) | Some(Status::Cancelled))
                        }
                    })
                })
            })
            .collect();

        Ok(ready)
    }

    /// Check and unblock tasks that were waiting on a now-resolved blocker
    /// Returns the paths of tasks that were unblocked
    pub fn unblock_waiting_tasks(&self, closed_task_id: &TaskId) -> Result<Vec<PathBuf>> {
        let blocked_tasks = self.list_by_status(Status::Blocked)?;
        let all_tasks = self.list_all()?;
        let mut unblocked = vec![];

        for (path, task) in blocked_tasks {
            // Check if this task was blocked by the closed task
            if !task.frontmatter.blocked_by.contains(closed_task_id) {
                continue;
            }

            // Check if ALL blockers are now resolved
            let all_resolved = task.frontmatter.blocked_by.iter().all(|blocker_id| {
                all_tasks.iter().any(|(blocker_path, t)| {
                    t.id() == blocker_id && {
                        let status = self.status_from_path(blocker_path);
                        matches!(status, Some(Status::Closed) | Some(Status::Cancelled))
                    }
                })
            });

            if all_resolved {
                // Move back to open
                let new_path = self.move_to_status(&path, Status::Open)?;
                unblocked.push(new_path);
            }
        }

        Ok(unblocked)
    }

    /// Get the tasks directory path
    pub fn tasks_dir(&self) -> &Path {
        &self.tasks_dir
    }
}
