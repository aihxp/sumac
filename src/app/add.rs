use std::path::PathBuf;

use sxmc::cli_surfaces::{self, AiClientProfile, AiCoverage, ArtifactMode};
use sxmc::error::Result;
use sxmc::output;
use sxmc::paths::InstallPaths;

use crate::{
    add_result_value, auto_detect_add_hosts, ensure_profile_ready_for_agent_docs,
    explicit_structured_format, host_label_list, print_preview_outcomes, print_write_outcomes,
    resolve_cli_ai_init_artifacts, AddResultContext,
};

use super::{
    onboarding::{emit_onboarding_header, MaterializeRequest, OnboardingService},
    CommandOutcome, GoldenPathRoute,
};

#[derive(Clone)]
pub(crate) struct AddRequest {
    pub(crate) command: String,
    pub(crate) depth: usize,
    pub(crate) install_paths: InstallPaths,
    pub(crate) skills_path: PathBuf,
    pub(crate) hosts: Vec<AiClientProfile>,
    pub(crate) preview: bool,
    pub(crate) allow_low_confidence: bool,
    pub(crate) allow_self: bool,
    pub(crate) pretty: bool,
    pub(crate) format: Option<output::StructuredOutputFormat>,
}

pub(crate) struct AddService {
    onboarding: OnboardingService,
}

impl AddService {
    pub(crate) fn new() -> Self {
        Self {
            onboarding: OnboardingService::new(),
        }
    }

    pub(crate) fn run(
        &self,
        route: GoldenPathRoute,
        request: AddRequest,
    ) -> Result<CommandOutcome> {
        match route {
            GoldenPathRoute::Legacy => self.run_legacy(request),
            GoldenPathRoute::Core => self.run_core(request),
        }
    }

    fn run_core(&self, request: AddRequest) -> Result<CommandOutcome> {
        let render_format = explicit_structured_format(request.format, request.pretty);
        let selection =
            self.onboarding
                .select_hosts(&request.install_paths, &request.hosts, request.preview);

        emit_onboarding_header(render_format, &selection, &request.install_paths);

        let onboarded = self.onboarding.inspect_and_materialize(MaterializeRequest {
            command: &request.command,
            depth: request.depth,
            allow_self: request.allow_self,
            allow_low_confidence: request.allow_low_confidence,
            install_paths: &request.install_paths,
            skills_path: &request.skills_path,
            selection: &selection,
        })?;

        if let Some(format) = render_format {
            let value = add_result_value(AddResultContext {
                install_paths: &request.install_paths,
                command: &request.command,
                profile: &onboarded.profile,
                hosts: &onboarded.selected_hosts,
                outcomes: &onboarded.outcomes,
                auto_detected_hosts: onboarded.auto_detected_hosts,
                preview_requested: request.preview,
                auto_previewed_due_to_missing_hosts: onboarded.auto_previewed_due_to_missing_hosts,
            });
            println!("{}", output::format_structured_value(&value, format));
        } else if selection.apply {
            print_write_outcomes(&onboarded.outcomes);
        } else {
            print_preview_outcomes(&onboarded.outcomes);
        }

        Ok(CommandOutcome::default())
    }

    fn run_legacy(&self, request: AddRequest) -> Result<CommandOutcome> {
        let render_format = explicit_structured_format(request.format, request.pretty);
        let auto_detected_hosts = request.hosts.is_empty();
        let selected_hosts = if auto_detected_hosts {
            auto_detect_add_hosts(&request.install_paths)
        } else {
            request.hosts.clone()
        };
        let has_selected_hosts = !selected_hosts.is_empty();
        let apply = has_selected_hosts && !request.preview;
        let auto_previewed_due_to_missing_hosts = !has_selected_hosts;

        if render_format.is_none() && apply {
            println!("Detected AI hosts: {}", host_label_list(&selected_hosts));
        } else if render_format.is_none() && has_selected_hosts {
            println!(
                "Previewing onboarding for AI hosts: {}",
                host_label_list(&selected_hosts)
            );
        } else if render_format.is_none() {
            println!(
                "No AI hosts detected for the {} install scope. Previewing the full onboarding plan instead.",
                request.install_paths.scope().as_str()
            );
            println!(
                "Tip: install a supported host runtime or pass --host <name> to apply directly."
            );
        }

        let profile =
            cli_surfaces::inspect_cli_with_depth(&request.command, request.allow_self, request.depth)?;
        ensure_profile_ready_for_agent_docs(&profile, request.allow_low_confidence)?;
        let (artifacts, selected_hosts) = resolve_cli_ai_init_artifacts(
            &profile,
            AiCoverage::Full,
            None,
            &selected_hosts,
            &request.install_paths,
            &request.skills_path,
            if apply {
                ArtifactMode::Apply
            } else {
                ArtifactMode::Preview
            },
        )?;

        let outcomes = if apply {
            cli_surfaces::materialize_artifacts_with_apply_selection(
                &artifacts,
                ArtifactMode::Apply,
                &request.install_paths,
                &selected_hosts,
            )?
        } else if has_selected_hosts {
            cli_surfaces::preview_artifacts_with_apply_selection(
                &artifacts,
                ArtifactMode::Apply,
                &request.install_paths,
                &selected_hosts,
            )?
        } else {
            cli_surfaces::preview_artifacts(
                &artifacts,
                ArtifactMode::Apply,
                &request.install_paths,
            )?
        };

        if let Some(format) = render_format {
            let value = add_result_value(AddResultContext {
                install_paths: &request.install_paths,
                command: &request.command,
                profile: &profile,
                hosts: &selected_hosts,
                outcomes: &outcomes,
                auto_detected_hosts,
                preview_requested: request.preview,
                auto_previewed_due_to_missing_hosts,
            });
            println!("{}", output::format_structured_value(&value, format));
        } else if apply {
            print_write_outcomes(&outcomes);
        } else {
            print_preview_outcomes(&outcomes);
        }

        Ok(CommandOutcome::default())
    }
}
