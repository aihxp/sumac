use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::error::{Result, SxmcError};

pub fn inspect_codebase(root: &Path, compact: bool) -> Result<Value> {
    let root = fs::canonicalize(root).map_err(|e| {
        SxmcError::Other(format!(
            "Failed to resolve codebase root '{}': {}",
            root.display(),
            e
        ))
    })?;

    let cargo_manifest = root.join("Cargo.toml");
    let package_manifest = root.join("package.json");
    let npm_package_manifest = root.join("packaging").join("npm").join("package.json");
    let workflows_dir = root.join(".github").join("workflows");

    let mut manifests = Vec::new();
    let mut task_runners = Vec::new();
    let mut entrypoints = Vec::new();
    let mut configs = Vec::new();

    if cargo_manifest.exists() {
        manifests.push(file_entry("cargo", &cargo_manifest, None));
        task_runners.push(json!({
            "name": "cargo",
            "kind": "rust",
            "path": cargo_manifest.display().to_string(),
        }));
        let cargo_contents = fs::read_to_string(&cargo_manifest).map_err(|e| {
            SxmcError::Other(format!(
                "Failed to read Cargo manifest '{}': {}",
                cargo_manifest.display(),
                e
            ))
        })?;
        let cargo_value: toml::Value = cargo_contents.parse().map_err(|e| {
            SxmcError::Other(format!(
                "Failed to parse Cargo manifest '{}': {}",
                cargo_manifest.display(),
                e
            ))
        })?;
        if let Some(package_name) = cargo_value
            .get("package")
            .and_then(|value| value.get("name"))
            .and_then(toml::Value::as_str)
        {
            entrypoints.push(json!({
                "kind": "cargo-package",
                "name": package_name,
                "path": cargo_manifest.display().to_string(),
            }));
        }
        if let Some(bins) = cargo_value.get("bin").and_then(toml::Value::as_array) {
            for bin in bins {
                entrypoints.push(json!({
                    "kind": "cargo-bin",
                    "name": bin.get("name").and_then(toml::Value::as_str).unwrap_or("<unnamed>"),
                    "path": bin.get("path").and_then(toml::Value::as_str).map(|p| root.join(p).display().to_string()).unwrap_or_else(|| cargo_manifest.display().to_string()),
                }));
            }
        }
    }

    if package_manifest.exists() {
        collect_package_manifest(
            &root,
            &package_manifest,
            "package-json",
            &mut manifests,
            &mut task_runners,
            &mut entrypoints,
        )?;
    }
    if npm_package_manifest.exists() {
        collect_package_manifest(
            &root,
            &npm_package_manifest,
            "package-json",
            &mut manifests,
            &mut task_runners,
            &mut entrypoints,
        )?;
    }

    if workflows_dir.exists() {
        for workflow in read_sorted_files(&workflows_dir)? {
            if matches!(
                workflow.extension().and_then(|value| value.to_str()),
                Some("yml") | Some("yaml")
            ) {
                configs.push(file_entry("github-workflow", &workflow, None));
            }
        }
        if !configs.is_empty() {
            task_runners.push(json!({
                "name": "github-actions",
                "kind": "workflow",
                "path": workflows_dir.display().to_string(),
            }));
        }
    }

    for relative in [
        "README.md",
        "LICENSE",
        ".cursor/rules/sxmc-cli-ai.md",
        ".github/copilot-instructions.md",
        "CLAUDE.md",
        "AGENTS.md",
        "GEMINI.md",
    ] {
        let path = root.join(relative);
        if path.exists() {
            configs.push(file_entry("project-config", &path, Some(relative)));
        }
    }

    let value = json!({
        "discovery_schema": "sxmc_discover_codebase_v1",
        "source_type": "codebase",
        "root": root.display().to_string(),
        "manifest_count": manifests.len(),
        "task_runner_count": task_runners.len(),
        "entrypoint_count": entrypoints.len(),
        "config_count": configs.len(),
        "manifests": manifests,
        "task_runners": task_runners,
        "entrypoints": entrypoints,
        "configs": configs,
    });

    if compact {
        Ok(json!({
            "discovery_schema": value["discovery_schema"],
            "source_type": value["source_type"],
            "root": value["root"],
            "manifest_count": value["manifest_count"],
            "task_runner_count": value["task_runner_count"],
            "entrypoint_count": value["entrypoint_count"],
            "config_count": value["config_count"],
            "manifest_kinds": summarize_names(value["manifests"].as_array()),
            "task_runner_names": summarize_names(value["task_runners"].as_array()),
            "entrypoint_names": summarize_names(value["entrypoints"].as_array()),
        }))
    } else {
        Ok(value)
    }
}

fn collect_package_manifest(
    root: &Path,
    manifest_path: &Path,
    manifest_kind: &str,
    manifests: &mut Vec<Value>,
    task_runners: &mut Vec<Value>,
    entrypoints: &mut Vec<Value>,
) -> Result<()> {
    manifests.push(file_entry(manifest_kind, manifest_path, None));
    let contents = fs::read_to_string(manifest_path).map_err(|e| {
        SxmcError::Other(format!(
            "Failed to read package manifest '{}': {}",
            manifest_path.display(),
            e
        ))
    })?;
    let value: Value = serde_json::from_str(&contents).map_err(|e| {
        SxmcError::Other(format!(
            "Failed to parse package manifest '{}': {}",
            manifest_path.display(),
            e
        ))
    })?;

    let package_name = value
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("<unnamed>");
    task_runners.push(json!({
        "name": package_name,
        "kind": "npm",
        "path": manifest_path.display().to_string(),
    }));
    if let Some(scripts) = value.get("scripts").and_then(Value::as_object) {
        for (name, command) in scripts {
            entrypoints.push(json!({
                "kind": "npm-script",
                "name": name,
                "command": command,
                "path": manifest_path.display().to_string(),
                "workspace_root": root.display().to_string(),
            }));
        }
    }
    Ok(())
}

fn summarize_names(values: Option<&Vec<Value>>) -> Value {
    let names = values
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    item.get("name")
                        .and_then(Value::as_str)
                        .or_else(|| item.get("kind").and_then(Value::as_str))
                })
                .map(|name| Value::String(name.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Value::Array(names)
}

fn file_entry(kind: &str, path: &Path, label: Option<&str>) -> Value {
    json!({
        "kind": kind,
        "name": label.unwrap_or_else(|| path.file_name().and_then(|value| value.to_str()).unwrap_or("<unknown>")),
        "path": path.display().to_string(),
    })
}

fn read_sorted_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = fs::read_dir(dir)
        .map_err(|e| SxmcError::Other(format!("Failed to read '{}': {}", dir.display(), e)))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}
