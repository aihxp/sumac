use serde_json::Value;

use sxmc::cli_surfaces::AiClientProfile;
use sxmc::error::Result;
use sxmc::output;
use sxmc::paths::InstallPaths;

use crate::{explicit_structured_format, format_sync_report, sync_saved_profiles_value};

use super::{CommandOutcome, GoldenPathRoute};

#[derive(Clone)]
pub(crate) struct SyncRequest {
    pub(crate) install_paths: InstallPaths,
    pub(crate) only_hosts: Vec<AiClientProfile>,
    pub(crate) skills_path: std::path::PathBuf,
    pub(crate) apply: bool,
    pub(crate) check: bool,
    pub(crate) allow_low_confidence: bool,
    pub(crate) pretty: bool,
    pub(crate) format: Option<output::StructuredOutputFormat>,
}

#[derive(Default, Clone, Copy)]
struct SyncAdapters;

impl SyncAdapters {
    fn collect(&self, request: &SyncRequest) -> Result<Value> {
        sync_saved_profiles_value(
            &request.install_paths,
            &request.only_hosts,
            &request.skills_path,
            request.apply,
            request.allow_low_confidence,
        )
    }
}

pub(crate) struct SyncService {
    adapters: SyncAdapters,
}

impl SyncService {
    pub(crate) fn new() -> Self {
        Self {
            adapters: SyncAdapters,
        }
    }

    pub(crate) fn run(
        &self,
        route: GoldenPathRoute,
        request: SyncRequest,
    ) -> Result<CommandOutcome> {
        match route {
            GoldenPathRoute::Legacy => self.run_legacy(request),
            GoldenPathRoute::Core => self.run_core(request),
        }
    }

    fn run_core(&self, request: SyncRequest) -> Result<CommandOutcome> {
        let value = self.adapters.collect(&request)?;
        render_sync_value(&value, request.pretty, request.format);
        Ok(sync_exit_outcome(&value, request.apply, request.check))
    }

    fn run_legacy(&self, request: SyncRequest) -> Result<CommandOutcome> {
        let value = self.adapters.collect(&request)?;
        render_sync_value(&value, request.pretty, request.format);
        Ok(sync_exit_outcome(&value, request.apply, request.check))
    }
}

fn sync_exit_outcome(value: &Value, apply: bool, check: bool) -> CommandOutcome {
    CommandOutcome {
        exit_code: if check
            && (value["blocked_count"].as_u64().unwrap_or(0) > 0
                || value["error_count"].as_u64().unwrap_or(0) > 0
                || (!apply && value["changed_count"].as_u64().unwrap_or(0) > 0))
        {
            Some(1)
        } else {
            None
        },
    }
}

fn render_sync_value(
    value: &Value,
    pretty: bool,
    format: Option<output::StructuredOutputFormat>,
) {
    if let Some(format) = explicit_structured_format(format, pretty) {
        println!("{}", output::format_structured_value(value, format));
    } else {
        println!("{}", format_sync_report(value));
    }
}
