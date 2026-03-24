use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

use crate::error::{Result, SxmcError};
use crate::paths;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheStats {
    pub path: PathBuf,
    pub entry_count: usize,
    pub total_bytes: u64,
    pub default_ttl_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheRecord {
    pub key: String,
    pub path: PathBuf,
    pub data: String,
}

/// A file-based cache with TTL support.
/// Stores entries in ~/.cache/sxmc/
pub struct Cache {
    dir: PathBuf,
    default_ttl: Duration,
}

#[derive(Serialize, Deserialize)]
struct CacheEntry {
    #[serde(default)]
    key: String,
    data: String,
    created_at: u64,
    ttl_secs: u64,
}

impl Cache {
    /// Create a new cache with the given TTL.
    pub fn new(ttl_secs: u64) -> Result<Self> {
        Self::with_dir(paths::cache_dir(), ttl_secs)
    }

    /// Create a cache rooted at an explicit directory.
    pub fn with_dir(dir: PathBuf, ttl_secs: u64) -> Result<Self> {
        std::fs::create_dir_all(&dir)
            .map_err(|e| SxmcError::Other(format!("Failed to create cache dir: {}", e)))?;

        Ok(Self {
            dir,
            default_ttl: Duration::from_secs(ttl_secs),
        })
    }

    /// Get a cached value by key, if it exists and hasn't expired.
    pub fn get(&self, key: &str) -> Option<String> {
        let path = self.key_path(key);
        let content = std::fs::read_to_string(&path).ok()?;
        let mut entry: CacheEntry = serde_json::from_str(&content).ok()?;

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()?
            .as_secs();

        if now - entry.created_at > entry.ttl_secs {
            // Expired — clean up
            let _ = std::fs::remove_file(&path);
            return None;
        }

        if entry.key.is_empty() {
            entry.key = key.to_string();
            if let Ok(json) = serde_json::to_string(&entry) {
                let _ = std::fs::write(&path, json);
            }
        }

        Some(entry.data)
    }

    /// Store a value in the cache.
    pub fn set(&self, key: &str, data: &str) -> Result<()> {
        self.set_with_ttl(key, data, self.default_ttl.as_secs())
    }

    /// Store a value with a custom TTL.
    pub fn set_with_ttl(&self, key: &str, data: &str, ttl_secs: u64) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| SxmcError::Other(format!("System time error: {}", e)))?
            .as_secs();

        let entry = CacheEntry {
            key: key.to_string(),
            data: data.to_string(),
            created_at: now,
            ttl_secs,
        };

        let json = serde_json::to_string(&entry)?;
        let path = self.key_path(key);
        std::fs::write(&path, json)
            .map_err(|e| SxmcError::Other(format!("Failed to write cache: {}", e)))?;

        Ok(())
    }

    /// Remove a cached entry.
    pub fn remove(&self, key: &str) {
        let _ = std::fs::remove_file(self.key_path(key));
    }

    /// Clear all cached entries.
    pub fn clear(&self) -> Result<()> {
        if self.dir.exists() {
            for entry in std::fs::read_dir(&self.dir)
                .map_err(|e| SxmcError::Other(format!("Failed to read cache dir: {}", e)))?
                .flatten()
            {
                let _ = std::fs::remove_file(entry.path());
            }
        }
        Ok(())
    }

    /// Remove cached entries whose original logical key matches the predicate.
    pub fn remove_matching<F>(&self, mut predicate: F) -> Result<usize>
    where
        F: FnMut(&str) -> bool,
    {
        let mut removed = 0usize;
        if self.dir.exists() {
            for entry in std::fs::read_dir(&self.dir)
                .map_err(|e| SxmcError::Other(format!("Failed to read cache dir: {}", e)))?
                .flatten()
            {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let Ok(content) = std::fs::read_to_string(&path) else {
                    continue;
                };
                let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) else {
                    continue;
                };
                if cache_entry.key.is_empty() {
                    continue;
                }
                if predicate(&cache_entry.key) && std::fs::remove_file(&path).is_ok() {
                    removed += 1;
                }
            }
        }
        Ok(removed)
    }

    pub fn records(&self) -> Result<Vec<CacheRecord>> {
        let mut records = Vec::new();
        if self.dir.exists() {
            for entry in std::fs::read_dir(&self.dir)
                .map_err(|e| SxmcError::Other(format!("Failed to read cache dir: {}", e)))?
                .flatten()
            {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let Ok(content) = std::fs::read_to_string(&path) else {
                    continue;
                };
                let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) else {
                    continue;
                };
                let key = if cache_entry.key.is_empty() {
                    String::new()
                } else {
                    cache_entry.key
                };
                records.push(CacheRecord {
                    key,
                    path,
                    data: cache_entry.data,
                });
            }
        }
        Ok(records)
    }

    /// Return summary information about the cache directory.
    pub fn stats(&self) -> Result<CacheStats> {
        let mut entry_count = 0usize;
        let mut total_bytes = 0u64;

        if self.dir.exists() {
            for entry in std::fs::read_dir(&self.dir)
                .map_err(|e| SxmcError::Other(format!("Failed to read cache dir: {}", e)))?
                .flatten()
            {
                let path = entry.path();
                if path.is_file() {
                    entry_count += 1;
                    total_bytes += entry.metadata().map(|meta| meta.len()).unwrap_or(0);
                }
            }
        }

        Ok(CacheStats {
            path: self.dir.clone(),
            entry_count,
            total_bytes,
            default_ttl_secs: self.default_ttl.as_secs(),
        })
    }

    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    fn key_path(&self, key: &str) -> PathBuf {
        // Hash the key for safe filenames
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        self.dir.join(format!("{:x}.json", hasher.finish()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn test_cache(ttl_secs: u64) -> Cache {
        let dir = tempdir().unwrap();
        let cache_dir = dir.path().to_path_buf();
        let _dir = dir.keep();
        Cache::with_dir(cache_dir, ttl_secs).unwrap()
    }

    #[test]
    fn test_cache_set_get() {
        let cache = test_cache(3600);
        let key = "test_cache_set_get";
        cache.set(key, "hello world").unwrap();
        assert_eq!(cache.get(key), Some("hello world".to_string()));
        cache.remove(key);
    }

    #[test]
    fn test_cache_miss() {
        let cache = test_cache(3600);
        assert_eq!(cache.get("nonexistent_key_12345"), None);
    }

    #[test]
    fn test_cache_expired() {
        let cache = test_cache(3600);
        let key = "test_cache_expired";
        // Set with 0 TTL — immediately expired
        cache.set_with_ttl(key, "expired data", 0).unwrap();
        // Sleep over 1 second to ensure the second-granularity timestamp advances
        std::thread::sleep(std::time::Duration::from_millis(1100));
        assert_eq!(cache.get(key), None);
    }

    #[test]
    fn test_cache_stats_reports_entries() {
        let cache = test_cache(3600);
        let key = "test_cache_stats_reports_entries";
        cache.set(key, "hello world").unwrap();

        let stats = cache.stats().unwrap();
        assert!(stats.entry_count >= 1);
        assert!(stats.total_bytes > 0);
        assert_eq!(stats.default_ttl_secs, 3600);

        cache.remove(key);
    }

    #[test]
    fn test_cache_remove_matching_removes_selected_entries() {
        let cache = test_cache(3600);
        let first = "cli-profile:first";
        let second = "cli-profile:second";
        cache.set(first, "hello").unwrap();
        cache.set(second, "world").unwrap();

        let removed = cache.remove_matching(|key| key.contains("first")).unwrap();
        assert_eq!(removed, 1);
        assert_eq!(cache.get(first), None);
        assert_eq!(cache.get(second), Some("world".to_string()));

        cache.remove(second);
    }
}
