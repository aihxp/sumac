use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Result, SxmcError};

/// A baked connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BakeConfig {
    pub name: String,
    pub source_type: SourceType,
    pub source: String,
    pub auth_headers: Vec<String>,
    pub env_vars: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Stdio,
    Http,
    Api,
    Spec,
    Graphql,
}

/// Store for baked configurations. Persists to ~/.config/sxmc/bakes.json
pub struct BakeStore {
    path: PathBuf,
    configs: HashMap<String, BakeConfig>,
}

impl BakeStore {
    /// Load the bake store from disk.
    pub fn load() -> Result<Self> {
        let dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("sxmc");

        std::fs::create_dir_all(&dir)
            .map_err(|e| SxmcError::Other(format!("Failed to create config dir: {}", e)))?;

        let path = dir.join("bakes.json");
        let configs = if path.exists() {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| SxmcError::Other(format!("Failed to read bakes: {}", e)))?;
            serde_json::from_str(&content)?
        } else {
            HashMap::new()
        };

        Ok(Self { path, configs })
    }

    /// Save the store to disk.
    fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.configs)?;
        std::fs::write(&self.path, json)
            .map_err(|e| SxmcError::Other(format!("Failed to write bakes: {}", e)))?;
        Ok(())
    }

    /// Create a new baked config.
    pub fn create(&mut self, config: BakeConfig) -> Result<()> {
        if self.configs.contains_key(&config.name) {
            return Err(SxmcError::Other(format!(
                "Bake '{}' already exists. Use update or remove first.",
                config.name
            )));
        }
        self.configs.insert(config.name.clone(), config);
        self.save()
    }

    /// Update an existing baked config.
    pub fn update(&mut self, config: BakeConfig) -> Result<()> {
        if !self.configs.contains_key(&config.name) {
            return Err(SxmcError::Other(format!(
                "Bake '{}' not found",
                config.name
            )));
        }
        self.configs.insert(config.name.clone(), config);
        self.save()
    }

    /// Remove a baked config.
    pub fn remove(&mut self, name: &str) -> Result<()> {
        if self.configs.remove(name).is_none() {
            return Err(SxmcError::Other(format!("Bake '{}' not found", name)));
        }
        self.save()
    }

    /// Get a baked config by name.
    pub fn get(&self, name: &str) -> Option<&BakeConfig> {
        self.configs.get(name)
    }

    /// List all baked configs.
    pub fn list(&self) -> Vec<&BakeConfig> {
        let mut configs: Vec<_> = self.configs.values().collect();
        configs.sort_by(|a, b| a.name.cmp(&b.name));
        configs
    }

    /// Show details for a baked config.
    pub fn show(&self, name: &str) -> Option<&BakeConfig> {
        self.configs.get(name)
    }
}

impl std::fmt::Display for BakeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?}: {})", self.name, self.source_type, self.source)?;
        if let Some(ref desc) = self.description {
            write!(f, " — {}", desc)?;
        }
        Ok(())
    }
}
