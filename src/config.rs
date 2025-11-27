use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Project-level configuration stored in .tasks/config.yaml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default_author: Option<String>,
}

impl Config {
    /// Load config from the .tasks directory
    pub fn load(tasks_dir: &Path) -> Self {
        let config_path = tasks_dir.join("config.yaml");
        if config_path.exists() {
            fs::read_to_string(&config_path)
                .ok()
                .and_then(|content| serde_yaml::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            Config::default()
        }
    }

    /// Save config to the .tasks directory
    pub fn save(&self, tasks_dir: &Path) -> Result<(), std::io::Error> {
        let config_path = tasks_dir.join("config.yaml");
        let yaml = serde_yaml::to_string(self).unwrap_or_default();
        fs::write(config_path, yaml)
    }

    /// Get the default author, falling back to git config
    pub fn get_author(&self) -> Option<String> {
        self.default_author.clone().or_else(|| {
            std::process::Command::new("git")
                .args(["config", "user.name"])
                .output()
                .ok()
                .and_then(|output| {
                    if output.status.success() {
                        String::from_utf8(output.stdout)
                            .ok()
                            .map(|s| s.trim().to_string())
                    } else {
                        None
                    }
                })
        })
    }
}
