use sxmc::cli_surfaces::{self, AiClientProfile, AiCoverage, ArtifactMode};
use sxmc::error::Result;
use sxmc::paths::InstallPaths;

use crate::{
    auto_detect_add_hosts, ensure_profile_ready_for_agent_docs, host_label_list,
    resolve_cli_ai_init_artifacts,
};

#[derive(Clone)]
pub(crate) struct OnboardingSelection {
    pub(crate) selected_hosts: Vec<AiClientProfile>,
    pub(crate) auto_detected_hosts: bool,
    pub(crate) has_selected_hosts: bool,
    pub(crate) apply: bool,
    pub(crate) auto_previewed_due_to_missing_hosts: bool,
}

#[derive(Clone)]
pub(crate) struct OnboardedTool {
    pub(crate) profile: cli_surfaces::CliSurfaceProfile,
    pub(crate) selected_hosts: Vec<AiClientProfile>,
    pub(crate) outcomes: Vec<cli_surfaces::WriteOutcome>,
    pub(crate) auto_detected_hosts: bool,
    pub(crate) auto_previewed_due_to_missing_hosts: bool,
}

pub(crate) struct MaterializeRequest<'a> {
    pub(crate) command: &'a str,
    pub(crate) depth: usize,
    pub(crate) allow_self: bool,
    pub(crate) allow_low_confidence: bool,
    pub(crate) install_paths: &'a InstallPaths,
    pub(crate) skills_path: &'a std::path::Path,
    pub(crate) selection: &'a OnboardingSelection,
}

#[derive(Default, Clone, Copy)]
struct OnboardingAdapters;

impl OnboardingAdapters {
    fn select_hosts(
        &self,
        install_paths: &InstallPaths,
        hosts: &[AiClientProfile],
        preview: bool,
    ) -> OnboardingSelection {
        let auto_detected_hosts = hosts.is_empty();
        let selected_hosts = if auto_detected_hosts {
            auto_detect_add_hosts(install_paths)
        } else {
            hosts.to_vec()
        };
        let has_selected_hosts = !selected_hosts.is_empty();
        let apply = has_selected_hosts && !preview;
        let auto_previewed_due_to_missing_hosts = !has_selected_hosts;
        OnboardingSelection {
            selected_hosts,
            auto_detected_hosts,
            has_selected_hosts,
            apply,
            auto_previewed_due_to_missing_hosts,
        }
    }

    fn inspect_and_materialize(&self, request: MaterializeRequest<'_>) -> Result<OnboardedTool> {
        let profile = cli_surfaces::inspect_cli_with_depth(
            request.command,
            request.allow_self,
            request.depth,
        )?;
        ensure_profile_ready_for_agent_docs(&profile, request.allow_low_confidence)?;
        let (artifacts, selected_hosts) = resolve_cli_ai_init_artifacts(
            &profile,
            AiCoverage::Full,
            None,
            &request.selection.selected_hosts,
            request.install_paths,
            request.skills_path,
            if request.selection.apply {
                ArtifactMode::Apply
            } else {
                ArtifactMode::Preview
            },
        )?;

        let outcomes = if request.selection.apply {
            cli_surfaces::materialize_artifacts_with_apply_selection(
                &artifacts,
                ArtifactMode::Apply,
                request.install_paths,
                &selected_hosts,
            )?
        } else if request.selection.has_selected_hosts {
            cli_surfaces::preview_artifacts_with_apply_selection(
                &artifacts,
                ArtifactMode::Apply,
                request.install_paths,
                &selected_hosts,
            )?
        } else {
            cli_surfaces::preview_artifacts(
                &artifacts,
                ArtifactMode::Apply,
                request.install_paths,
            )?
        };

        Ok(OnboardedTool {
            profile,
            selected_hosts,
            outcomes,
            auto_detected_hosts: request.selection.auto_detected_hosts,
            auto_previewed_due_to_missing_hosts: request
                .selection
                .auto_previewed_due_to_missing_hosts,
        })
    }
}

pub(crate) struct OnboardingService {
    adapters: OnboardingAdapters,
}

impl OnboardingService {
    pub(crate) fn new() -> Self {
        Self {
            adapters: OnboardingAdapters,
        }
    }

    pub(crate) fn select_hosts(
        &self,
        install_paths: &InstallPaths,
        hosts: &[AiClientProfile],
        preview: bool,
    ) -> OnboardingSelection {
        self.adapters.select_hosts(install_paths, hosts, preview)
    }

    pub(crate) fn inspect_and_materialize(
        &self,
        request: MaterializeRequest<'_>,
    ) -> Result<OnboardedTool> {
        self.adapters.inspect_and_materialize(request)
    }
}

pub(crate) fn emit_onboarding_header(
    render_format: Option<sxmc::output::StructuredOutputFormat>,
    selection: &OnboardingSelection,
    install_paths: &InstallPaths,
) {
    if render_format.is_none() && selection.apply {
        println!(
            "Detected AI hosts: {}",
            host_label_list(&selection.selected_hosts)
        );
    } else if render_format.is_none() && selection.has_selected_hosts {
        println!(
            "Previewing onboarding for AI hosts: {}",
            host_label_list(&selection.selected_hosts)
        );
    } else if render_format.is_none() {
        println!(
            "No AI hosts detected for the {} install scope. Previewing the full onboarding plan instead.",
            install_paths.scope().as_str()
        );
        println!(
            "Tip: install a supported host runtime or pass --host <name> to apply directly."
        );
    }
}
