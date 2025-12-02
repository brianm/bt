use crate::id::TaskId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Task status - derived from filesystem location, not stored in file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    Open,
    #[serde(rename = "in-progress")]
    InProgress,
    Blocked,
    Closed,
    Cancelled,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => write!(f, "open"),
            Status::InProgress => write!(f, "in-progress"),
            Status::Blocked => write!(f, "blocked"),
            Status::Closed => write!(f, "closed"),
            Status::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
            Priority::Critical => write!(f, "critical"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

/// YAML frontmatter for a task (status is NOT stored here - it's derived from directory)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFrontmatter {
    pub title: String,
    pub id: TaskId,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default)]
    pub priority: Priority,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_by: Vec<TaskId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<TaskId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<TaskId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<TaskId>,
}

/// A complete task with frontmatter and body
#[derive(Debug, Clone)]
pub struct Task {
    pub frontmatter: TaskFrontmatter,
    pub body: String,
    pub log: String,
}

impl Task {
    /// Create a new task with the given title
    pub fn new(title: impl Into<String>, author: Option<String>) -> Self {
        let title = title.into();
        let now = Utc::now();
        let id = TaskId::new();

        let frontmatter = TaskFrontmatter {
            title,
            id,
            created: now,
            updated: now,
            author,
            priority: Priority::Medium,
            tags: vec![],
            blocked_by: vec![],
            blocks: vec![],
            parent: None,
            children: vec![],
        };

        let log = format!(
            "---\n# Log: {} {}\n\nCreated task.\n",
            now.format("%Y-%m-%dT%H:%M:%SZ"),
            frontmatter.author.as_deref().unwrap_or("unknown")
        );

        Task {
            frontmatter,
            body: String::new(),
            log,
        }
    }

    /// Parse a task from markdown content
    pub fn parse(content: &str) -> Result<Self, String> {
        // Split on YAML frontmatter delimiters
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err("Invalid task format: missing YAML frontmatter".into());
        }

        let yaml = parts[1].trim();
        let rest = parts[2];

        let frontmatter: TaskFrontmatter =
            serde_yaml::from_str(yaml).map_err(|e| format!("Failed to parse frontmatter: {}", e))?;

        // Split body and log section
        let (body, log) = if let Some(log_start) = rest.find("\n---\n## Log") {
            let body = rest[..log_start].trim().to_string();
            let log = rest[log_start + 5..].trim_start_matches("## Log").trim().to_string();
            (body, log)
        } else if let Some(log_start) = rest.find("\n## Log") {
            let body = rest[..log_start].trim().to_string();
            let log = rest[log_start..].trim_start_matches("## Log").trim().to_string();
            (body, log)
        } else {
            (rest.trim().to_string(), String::new())
        };

        Ok(Task {
            frontmatter,
            body,
            log,
        })
    }

    /// Serialize the task to markdown
    pub fn to_markdown(&self) -> String {
        let yaml = serde_yaml::to_string(&self.frontmatter).unwrap_or_default();
        let yaml = yaml.trim();

        let mut md = format!("---\n{}\n---\n", yaml);

        if !self.body.is_empty() {
            md.push_str(&format!("\n{}\n", self.body));
        }

        md.push_str("\n---\n## Log\n\n");
        md.push_str(&self.log);

        md
    }

    /// Add a log entry
    pub fn add_log(&mut self, message: &str, author: Option<&str>) {
        let now = Utc::now();
        let author = author
            .or(self.frontmatter.author.as_deref())
            .unwrap_or("unknown");

        let entry = format!(
            "\n---\n# Log: {} {}\n\n{}\n",
            now.format("%Y-%m-%dT%H:%M:%SZ"),
            author,
            message
        );

        self.log.push_str(&entry);
        self.frontmatter.updated = now;
    }

    /// Get the task ID
    pub fn id(&self) -> &TaskId {
        &self.frontmatter.id
    }

    /// Get the task title
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the task priority
    pub fn priority(&self) -> Priority {
        self.frontmatter.priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = Task::new("Test task", Some("brian".into()));
        assert_eq!(task.title(), "Test task");
        assert_eq!(task.id().full().len(), 8); // 8 base32 chars
    }

    #[test]
    fn test_roundtrip() {
        let task = Task::new("Test task", Some("brian".into()));
        let md = task.to_markdown();
        let parsed = Task::parse(&md).unwrap();

        assert_eq!(parsed.title(), task.title());
        assert_eq!(parsed.id().full(), task.id().full());
    }
}
