use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

use crate::error::{Result, SxmcError};

/// Result of executing a script
#[derive(Debug)]
pub struct ExecResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

pub fn script_command(script_path: &Path) -> (std::path::PathBuf, Vec<String>) {
    #[cfg(windows)]
    {
        fn bash_script_path(script_path: &Path) -> String {
            let raw = script_path.to_string_lossy();
            let trimmed = raw.strip_prefix(r"\\?\").unwrap_or(&raw);
            trimmed.replace('\\', "/")
        }

        let uses_bash = script_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "sh" | "bash"))
            .unwrap_or(false);
        if uses_bash {
            let script_arg = bash_script_path(script_path);
            for candidate in [
                r"C:\Program Files\Git\bin\bash.exe",
                r"C:\Program Files\Git\usr\bin\bash.exe",
            ] {
                let path = std::path::PathBuf::from(candidate);
                if path.exists() {
                    return (path, vec![script_arg.clone()]);
                }
            }
            return (std::path::PathBuf::from("bash"), vec![script_arg]);
        }
    }

    (script_path.to_path_buf(), Vec::new())
}

/// Execute a script with arguments, capturing output.
pub async fn execute_script(
    script_path: &Path,
    args: &[&str],
    working_dir: &Path,
    timeout_secs: u64,
) -> Result<ExecResult> {
    let (executable, mut launcher_args) = script_command(script_path);
    launcher_args.extend(args.iter().map(|arg| arg.to_string()));

    let mut cmd = Command::new(executable);
    cmd.args(&launcher_args)
        .current_dir(working_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let result = timeout(Duration::from_secs(timeout_secs), cmd.output())
        .await
        .map_err(|_| SxmcError::TimeoutError(timeout_secs))?
        .map_err(|e| {
            SxmcError::ExecutionError(format!("Failed to run {}: {}", script_path.display(), e))
        })?;

    Ok(ExecResult {
        stdout: String::from_utf8_lossy(&result.stdout).to_string(),
        stderr: String::from_utf8_lossy(&result.stderr).to_string(),
        exit_code: result.status.code().unwrap_or(-1),
    })
}

/// Execute an arbitrary command with arguments, capturing output.
pub async fn execute_command(
    executable: &str,
    args: &[String],
    working_dir: Option<&Path>,
    timeout_secs: u64,
) -> Result<ExecResult> {
    let mut cmd = Command::new(executable);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(working_dir) = working_dir {
        cmd.current_dir(working_dir);
    }

    let result = timeout(Duration::from_secs(timeout_secs), cmd.output())
        .await
        .map_err(|_| SxmcError::TimeoutError(timeout_secs))?
        .map_err(|e| SxmcError::ExecutionError(format!("Failed to run {}: {}", executable, e)))?;

    Ok(ExecResult {
        stdout: String::from_utf8_lossy(&result.stdout).to_string(),
        stderr: String::from_utf8_lossy(&result.stderr).to_string(),
        exit_code: result.status.code().unwrap_or(-1),
    })
}
