use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use serde_json::{json, Value};

use crate::cli_surfaces::model::{
    AuthRequirement, CliSurfaceProfile, ConfidenceLevel, ConfidenceNote, EnvironmentRequirement,
    OutputBehavior, ProfileExample, ProfileOption, ProfilePositional, ProfileSource,
    ProfileSubcommand, Provenance, Workflow, PROFILE_SCHEMA,
};
use crate::error::{Result, SxmcError};

pub fn parse_command_spec(command: &str) -> Result<Vec<String>> {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    if trimmed.starts_with('[') {
        return serde_json::from_str::<Vec<String>>(trimmed).map_err(|e| {
            SxmcError::Other(format!(
                "Invalid command JSON array. Expected [\"cmd\", \"arg1\", ...]: {}",
                e
            ))
        });
    }

    #[cfg(windows)]
    {
        if let Some(parts) = parse_windows_command_spec(trimmed) {
            return Ok(parts);
        }
        return Ok(trimmed.split_whitespace().map(str::to_string).collect());
    }

    #[cfg(not(windows))]
    shlex::split(trimmed).ok_or_else(|| {
        SxmcError::Other(
            "Invalid command string. Use shell-style quoting or a JSON array command spec.".into(),
        )
    })
}

#[cfg(windows)]
fn parse_windows_command_spec(command: &str) -> Option<Vec<String>> {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return Some(Vec::new());
    }

    if let Some(rest) = trimmed.strip_prefix('"') {
        let quote_end = rest.find('"')?;
        let executable = &rest[..quote_end];
        let args = rest[quote_end + 1..].trim();
        let mut parts = vec![executable.to_string()];
        parts.extend(args.split_whitespace().map(str::to_string));
        return Some(parts);
    }

    let executable_pattern = Regex::new(r"(?i)^(.+?\.(exe|cmd|bat|ps1))(?:\s+(.*))?$").ok()?;
    let captures = executable_pattern.captures(trimmed)?;
    let executable = captures.get(1)?.as_str();
    let mut parts = vec![executable.to_string()];
    if let Some(args) = captures.get(3) {
        parts.extend(args.as_str().split_whitespace().map(str::to_string));
    }
    Some(parts)
}

pub fn inspect_cli(command_spec: &str, allow_self: bool) -> Result<CliSurfaceProfile> {
    let parts = parse_command_spec(command_spec)?;
    if parts.is_empty() {
        return Err(SxmcError::Other("Empty command spec".into()));
    }

    let executable = &parts[0];
    let command_name = Path::new(executable)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(executable)
        .to_string();

    if !allow_self && is_self_command(&command_name) {
        return Err(SxmcError::Other(
            "Refusing to inspect sxmc itself without --allow-self".into(),
        ));
    }

    let help_text = read_help_text(&parts)?;
    Ok(parse_help_text(&command_name, executable, &help_text))
}

pub fn load_profile(path: &Path) -> Result<CliSurfaceProfile> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

pub fn profile_value(profile: &CliSurfaceProfile) -> Value {
    serde_json::to_value(profile).unwrap_or_else(|_| json!({}))
}

fn is_self_command(command_name: &str) -> bool {
    let lowered = command_name.to_ascii_lowercase();
    lowered == "sxmc" || lowered == "sxmc.exe"
}

fn read_help_text(parts: &[String]) -> Result<String> {
    let mut command = Command::new(&parts[0]);
    if parts.len() > 1 {
        command.args(&parts[1..]);
    }
    command.arg("--help");
    let output = command
        .output()
        .map_err(|e| SxmcError::Other(format!("Failed to run '{} --help': {}", parts[0], e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let text = if !stdout.trim().is_empty() {
        stdout
    } else {
        stderr
    };

    if !output.status.success() && text.trim().is_empty() {
        return Err(SxmcError::Other(format!(
            "Command '{}' did not return readable help output",
            parts[0]
        )));
    }

    Ok(text)
}

fn parse_help_text(command_name: &str, source_identifier: &str, help: &str) -> CliSurfaceProfile {
    let lines: Vec<&str> = help.lines().collect();
    let summary = lines
        .iter()
        .map(|line| line.trim())
        .find(|line| !line.is_empty())
        .unwrap_or(command_name)
        .to_string();

    let description = parse_description(&lines);
    let subcommands = parse_subcommands(&lines);
    let options = parse_options(&lines);
    let positionals = parse_positionals(&lines, command_name);
    let examples = parse_examples(&lines, command_name);
    let (auth, environment) = infer_requirements(help);
    let workflows = infer_workflows(&subcommands);
    let output_behavior = infer_output_behavior(help);
    let mut confidence_notes = vec![ConfidenceNote {
        level: ConfidenceLevel::Medium,
        summary: "This profile was inferred from help output and may omit dynamic or plugin-provided behavior.".into(),
    }];
    if examples.is_empty() {
        confidence_notes.push(ConfidenceNote {
            level: ConfidenceLevel::Low,
            summary: "No examples were detected in help output; generated agent guidance may need manual examples.".into(),
        });
    }

    CliSurfaceProfile {
        profile_schema: PROFILE_SCHEMA.into(),
        command: command_name.into(),
        summary,
        description,
        source: ProfileSource {
            kind: "cli".into(),
            identifier: source_identifier.into(),
        },
        subcommands,
        options,
        positionals,
        examples,
        auth,
        environment,
        output_behavior,
        workflows,
        confidence_notes,
        provenance: Provenance {
            generated_by: "sxmc".into(),
            generator_version: env!("CARGO_PKG_VERSION").into(),
            source_kind: "cli".into(),
            source_identifier: source_identifier.into(),
            profile_schema: PROFILE_SCHEMA.into(),
            generation_depth: 0,
            generated_at: now_string(),
        },
    }
}

fn parse_description(lines: &[&str]) -> Option<String> {
    let mut description = Vec::new();
    let mut started = false;
    for line in lines.iter().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if started {
                break;
            }
            continue;
        }
        if is_section_heading(trimmed) || trimmed.starts_with("Usage:") {
            break;
        }
        started = true;
        description.push(trimmed.to_string());
    }
    if description.is_empty() {
        None
    } else {
        Some(description.join(" "))
    }
}

fn parse_subcommands(lines: &[&str]) -> Vec<ProfileSubcommand> {
    parse_table_section(lines, &["commands", "subcommands", "available commands"])
        .into_iter()
        .map(|(name, summary)| ProfileSubcommand {
            name,
            summary,
            confidence: ConfidenceLevel::High,
        })
        .collect()
}

fn parse_options(lines: &[&str]) -> Vec<ProfileOption> {
    let mut options = Vec::new();
    let mut in_options = false;
    let regex = Regex::new(
        r"^\s*(?:(-[A-Za-z0-9])(?:,\s*)?)?(--[A-Za-z0-9][A-Za-z0-9-]*)(?:[ =]([A-Z<>\[\]\-_|]+))?\s{2,}(.*)$",
    )
    .unwrap();
    let short_only_regex =
        Regex::new(r"^\s*(-[A-Za-z0-9])(?:[ =]([A-Z<>\[\]\-_|]+))?\s{2,}(.*)$").unwrap();

    for line in lines {
        let trimmed = line.trim_end();
        if trimmed.trim().is_empty() {
            if in_options {
                break;
            }
            continue;
        }
        if is_named_section(trimmed, &["options", "flags"]) {
            in_options = true;
            continue;
        }
        if !in_options {
            continue;
        }
        if is_section_heading(trimmed.trim()) {
            break;
        }
        if let Some(caps) = regex.captures(trimmed) {
            options.push(ProfileOption {
                name: caps
                    .get(2)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default(),
                short: caps.get(1).map(|m| m.as_str().to_string()),
                value_name: caps
                    .get(3)
                    .map(|m| m.as_str().trim_matches(&['<', '>'][..]).to_string()),
                required: false,
                summary: caps.get(4).map(|m| m.as_str().trim().to_string()),
                confidence: ConfidenceLevel::High,
            });
        } else if let Some(caps) = short_only_regex.captures(trimmed) {
            options.push(ProfileOption {
                name: caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default(),
                short: caps.get(1).map(|m| m.as_str().to_string()),
                value_name: caps
                    .get(2)
                    .map(|m| m.as_str().trim_matches(&['<', '>'][..]).to_string()),
                required: false,
                summary: caps.get(3).map(|m| m.as_str().trim().to_string()),
                confidence: ConfidenceLevel::Medium,
            });
        }
    }
    options
}

fn parse_positionals(lines: &[&str], command_name: &str) -> Vec<ProfilePositional> {
    let usage_line = lines
        .iter()
        .find_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("Usage:") {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .unwrap_or_default();

    if usage_line.is_empty() {
        return Vec::new();
    }

    usage_line
        .split_whitespace()
        .skip_while(|token| *token != command_name && !token.ends_with(command_name))
        .skip(1)
        .filter_map(|token| {
            if token.starts_with('-') || token.starts_with('[') || token == "[COMMAND]" {
                return None;
            }
            if !(token.starts_with('<') && token.ends_with('>')
                || token
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c == '_' || c == '-'))
            {
                return None;
            }
            let name = token.trim_matches(&['<', '>'][..]).trim_matches('.');
            if name.is_empty() {
                return None;
            }
            Some(ProfilePositional {
                name: name.to_ascii_lowercase(),
                required: true,
                summary: None,
                confidence: ConfidenceLevel::Medium,
            })
        })
        .collect()
}

fn parse_examples(lines: &[&str], command_name: &str) -> Vec<ProfileExample> {
    let mut examples = Vec::new();
    let mut in_examples = false;
    for line in lines {
        let trimmed = line.trim_end();
        let stripped = trimmed.trim();
        if stripped.is_empty() {
            if in_examples && !examples.is_empty() {
                break;
            }
            continue;
        }
        if is_named_section(stripped, &["examples", "example"]) {
            in_examples = true;
            continue;
        }
        if !in_examples {
            continue;
        }
        if is_section_heading(stripped) {
            break;
        }
        if stripped.starts_with(command_name) || stripped.starts_with('$') {
            examples.push(ProfileExample {
                command: stripped.trim_start_matches("$ ").to_string(),
                summary: None,
                confidence: ConfidenceLevel::High,
            });
        }
    }
    examples
}

fn infer_requirements(help: &str) -> (Vec<AuthRequirement>, Vec<EnvironmentRequirement>) {
    let mut auth = Vec::new();
    let mut environment = Vec::new();
    let mut seen_env = std::collections::BTreeSet::new();

    if help.to_ascii_lowercase().contains("login")
        || help.to_ascii_lowercase().contains("authenticate")
        || help.to_ascii_lowercase().contains("auth")
    {
        auth.push(AuthRequirement {
            kind: "interactive".into(),
            summary:
                "Help output mentions login/authentication, so interactive setup may be required."
                    .into(),
        });
    }

    let env_regex = Regex::new(r"\b([A-Z][A-Z0-9_]{2,})\b").unwrap();
    for capture in env_regex.captures_iter(help) {
        let name = capture.get(1).map(|m| m.as_str()).unwrap_or_default();
        if (name.ends_with("_TOKEN")
            || name.ends_with("_KEY")
            || name.ends_with("_SECRET")
            || name == "TOKEN")
            && seen_env.insert(name.to_string())
        {
            environment.push(EnvironmentRequirement {
                name: name.into(),
                summary: Some(
                    "Detected in help output; likely needed for auth or configuration.".into(),
                ),
                required: true,
            });
            auth.push(AuthRequirement {
                kind: "env_var".into(),
                summary: format!("Help output mentions environment variable `{}`.", name),
            });
        }
    }

    (auth, environment)
}

fn infer_workflows(subcommands: &[ProfileSubcommand]) -> Vec<Workflow> {
    if subcommands.is_empty() {
        return Vec::new();
    }
    let steps = subcommands
        .iter()
        .take(3)
        .map(|subcommand| format!("Use `{}` for {}", subcommand.name, subcommand.summary))
        .collect();
    vec![Workflow {
        name: "Common command flow".into(),
        steps,
        confidence: ConfidenceLevel::Medium,
    }]
}

fn infer_output_behavior(help: &str) -> OutputBehavior {
    let lowered = help.to_ascii_lowercase();
    OutputBehavior {
        stdout_style: if lowered.contains("--json") || lowered.contains(" json ") {
            "mixed".into()
        } else {
            "plain_text".into()
        },
        stderr_usage: "Unknown; inspect live behavior before piping stderr into machine parsers."
            .into(),
        machine_friendly: lowered.contains("--json") || lowered.contains("json output"),
    }
}

fn parse_table_section(lines: &[&str], headings: &[&str]) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    let mut in_section = false;

    for line in lines {
        let trimmed = line.trim_end();
        let stripped = trimmed.trim();

        if stripped.is_empty() {
            if in_section && !rows.is_empty() {
                break;
            }
            continue;
        }

        if is_named_section(stripped, headings) {
            in_section = true;
            continue;
        }

        if !in_section {
            continue;
        }

        if is_section_heading(stripped) {
            break;
        }

        let columns: Vec<&str> = stripped
            .split("  ")
            .filter(|chunk| !chunk.trim().is_empty())
            .collect();

        if columns.len() >= 2 {
            rows.push((
                columns[0].trim().to_string(),
                columns[1..].join(" ").trim().to_string(),
            ));
        }
    }

    rows
}

fn is_named_section(line: &str, headings: &[&str]) -> bool {
    let normalized = line.trim_end_matches(':').to_ascii_lowercase();
    headings.iter().any(|heading| normalized == *heading)
}

fn is_section_heading(line: &str) -> bool {
    line.ends_with(':')
}

fn now_string() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{}", seconds)
}

#[cfg(test)]
mod tests {
    use super::parse_command_spec;

    #[test]
    fn parse_json_array_command_spec() {
        let parsed = parse_command_spec(r#"["sxmc","serve","--paths","tests/fixtures"]"#).unwrap();
        assert_eq!(parsed, vec!["sxmc", "serve", "--paths", "tests/fixtures"]);
    }
}
