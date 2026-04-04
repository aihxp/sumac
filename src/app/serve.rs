use std::path::PathBuf;

use sxmc::cli_surfaces::{AiClientProfile, ArtifactMode};
use sxmc::error::Result;
use sxmc::server::{self, HttpServeLimits};

use crate::{
    apply_runtime_registration, parse_headers, parse_optional_secret, resolve_generation_root,
    resolve_paths, RuntimeMcpRegistration,
};

use super::CommandOutcome;

#[derive(Clone)]
pub(crate) struct ServeRequest {
    pub(crate) paths: Option<Vec<PathBuf>>,
    pub(crate) discovery_snapshots: Vec<PathBuf>,
    pub(crate) discovery_tool_manifests: Vec<PathBuf>,
    pub(crate) watch: bool,
    pub(crate) transport: String,
    pub(crate) port: u16,
    pub(crate) host: String,
    pub(crate) require_headers: Vec<String>,
    pub(crate) bearer_token: Option<String>,
    pub(crate) max_concurrency: usize,
    pub(crate) max_request_bytes: usize,
    pub(crate) register_hosts: Vec<AiClientProfile>,
    pub(crate) register_root: Option<PathBuf>,
    pub(crate) register_mode: ArtifactMode,
    pub(crate) register_name: Option<String>,
}

pub(crate) struct ServeService;

impl ServeService {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn run(&self, request: ServeRequest) -> Result<CommandOutcome> {
        let search_paths = resolve_paths(request.paths);

        if !request.register_hosts.is_empty() && request.transport != "stdio" {
            return Err(sxmc::error::SxmcError::Other(
                "Automatic MCP registration currently supports stdio transport only for `sxmc serve`. Use `--transport stdio` or register the HTTP endpoint manually.".into(),
            ));
        }

        if !request.register_hosts.is_empty() {
            let root = resolve_generation_root(request.register_root)?;
            let search_paths_arg = search_paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(",");
            let mut args = vec!["serve".to_string()];
            if !search_paths_arg.is_empty() {
                args.push("--paths".into());
                args.push(search_paths_arg);
            }
            if !request.discovery_snapshots.is_empty() {
                let snapshot_arg = request
                    .discovery_snapshots
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                args.push("--discovery-snapshot".into());
                args.push(snapshot_arg);
            }
            if !request.discovery_tool_manifests.is_empty() {
                let manifest_arg = request
                    .discovery_tool_manifests
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                args.push("--discovery-tool-manifest".into());
                args.push(manifest_arg);
            }
            if request.watch {
                args.push("--watch".into());
            }
            let registration = RuntimeMcpRegistration::Stdio {
                command: "sxmc".into(),
                args,
            };
            apply_runtime_registration(
                &root,
                &request.register_hosts,
                request.register_mode,
                request.register_name.as_deref(),
                "sxmc-serve",
                &registration,
            )?;
        }

        let required_headers = parse_headers(&request.require_headers)?;
        let bearer_token = parse_optional_secret(request.bearer_token)?;
        let limits = HttpServeLimits {
            max_concurrency: request.max_concurrency,
            max_request_body_bytes: request.max_request_bytes,
        };

        match request.transport.as_str() {
            "stdio" => {
                if !required_headers.is_empty() || bearer_token.is_some() {
                    eprintln!("[sxmc] Warning: remote auth flags are ignored for stdio transport");
                }
                server::serve_stdio(
                    &search_paths,
                    &request.discovery_snapshots,
                    &request.discovery_tool_manifests,
                    request.watch,
                )
                .await?;
            }
            "http" | "sse" => {
                server::serve_http(
                    &search_paths,
                    &request.discovery_snapshots,
                    &request.discovery_tool_manifests,
                    &request.host,
                    request.port,
                    &required_headers,
                    bearer_token.as_deref(),
                    request.watch,
                    limits,
                )
                .await?;
            }
            other => {
                eprintln!("[sxmc] Unknown transport: {}", other);
                return Ok(CommandOutcome { exit_code: Some(1) });
            }
        }

        Ok(CommandOutcome::default())
    }
}
