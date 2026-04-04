use std::io::IsTerminal;

use serde_json::Value;

use sxmc::cli_surfaces::AiClientProfile;
use sxmc::error::Result;
use sxmc::output;
use sxmc::paths::InstallPaths;

use crate::{
    print_status_report, status_has_unhealthy_baked_health, status_value_with_health,
};

use super::{CommandOutcome, GoldenPathRoute};

#[derive(Clone)]
pub(crate) struct StatusRequest {
    pub(crate) install_paths: InstallPaths,
    pub(crate) only_hosts: Vec<AiClientProfile>,
    pub(crate) compare_hosts: Vec<AiClientProfile>,
    pub(crate) health: bool,
    pub(crate) exit_code: bool,
    pub(crate) human: bool,
    pub(crate) pretty: bool,
    pub(crate) format: Option<output::StructuredOutputFormat>,
}

#[derive(Default, Clone, Copy)]
struct StatusAdapters;

impl StatusAdapters {
    async fn collect(&self, request: &StatusRequest) -> Result<Value> {
        status_value_with_health(
            &request.install_paths,
            &request.only_hosts,
            &request.compare_hosts,
            request.health,
        )
        .await
    }
}

pub(crate) struct StatusService {
    adapters: StatusAdapters,
}

impl StatusService {
    pub(crate) fn new() -> Self {
        Self {
            adapters: StatusAdapters,
        }
    }

    pub(crate) async fn run(
        &self,
        route: GoldenPathRoute,
        request: StatusRequest,
    ) -> Result<CommandOutcome> {
        match route {
            GoldenPathRoute::Legacy => self.run_legacy(request).await,
            GoldenPathRoute::Core => self.run_core(request).await,
        }
    }

    async fn run_core(&self, request: StatusRequest) -> Result<CommandOutcome> {
        let value = self.adapters.collect(&request).await?;
        render_status_value(&value, request.human, request.pretty, request.format);
        Ok(CommandOutcome {
            exit_code: if request.exit_code && status_has_unhealthy_baked_health(&value) {
                Some(1)
            } else {
                None
            },
        })
    }

    async fn run_legacy(&self, request: StatusRequest) -> Result<CommandOutcome> {
        let value = self.adapters.collect(&request).await?;
        render_status_value(&value, request.human, request.pretty, request.format);
        Ok(CommandOutcome {
            exit_code: if request.exit_code && status_has_unhealthy_baked_health(&value) {
                Some(1)
            } else {
                None
            },
        })
    }
}

fn render_status_value(
    value: &Value,
    human: bool,
    pretty: bool,
    format: Option<output::StructuredOutputFormat>,
) {
    if human || (format.is_none() && !pretty && std::io::stdout().is_terminal()) {
        print_status_report(value);
    } else if let Some(format) = output::prefer_structured_output(format, pretty) {
        println!("{}", output::format_structured_value(value, format));
    } else {
        let format = output::resolve_structured_format(format, pretty);
        println!("{}", output::format_structured_value(value, format));
    }
}
