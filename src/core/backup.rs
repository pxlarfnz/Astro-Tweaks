use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRecord {
    pub category: String,
    pub target: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Backup {
    pub records: Vec<ChangeRecord>,
}

impl Backup {
    pub fn path() -> PathBuf {
        PathBuf::from(r"C:\ProgramData\VividTweaks\backup.json")
    }

    pub fn load() -> Self {
        fs::read_to_string(Self::path())
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(
            &path,
            serde_json::to_string_pretty(self)?,
        )
        .with_context(|| format!("Failed to write {}", path.display()))
    }

    pub fn record(
        &mut self,
        category: &str,
        target: &str,
        old_value: Option<String>,
        new_value: Option<String>,
    ) {
        self.records.push(ChangeRecord {
            category: category.to_string(),
            target: target.to_string(),
            old_value,
            new_value,
        });
    }
}