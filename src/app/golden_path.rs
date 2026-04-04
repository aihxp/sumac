use std::path::PathBuf;

use serde_json::json;

use sxmc::cli_surfaces::{self, AiClientProfile, AiCoverage, ArtifactMode};
use sxmc::error::Result;
use sxmc::output;
use sxmc::paths::InstallPaths;

use super::{status::StatusRequest, sync::SyncRequest, CommandOutcome, GoldenPathRoute};

use crate::{
    add_result_value, auto_detect_add_hosts, detect_setup_tools,
    ensure_profile_ready_for_agent_docs, explicit_structured_format,
    host_label_list, print_preview_outcomes, print_write_outcomes, resolve_cli_ai_init_artifacts,
    setup_result_value, AddResultContext, SetupResultContext,
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

#[derive(Clone)]
pub(crate) struct SetupRequest {
    pub(crate) tools: Vec<String>,
    pub(crate) limit: usize,
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

#[derive(Clone)]
struct OnboardingSelection {
    selected_hosts: Vec<AiClientProfile>,
    auto_detected_hosts: bool,
    has_selected_hosts: bool,
    apply: bool,
    auto_previewed_due_to_missing_hosts: bool,
}

#[derive(Clone)]
struct OnboardedTool {
    profile: cli_surfaces::CliSurfaceProfile,
    selected_hosts: Vec<AiClientProfile>,
    outcomes: Vec<cli_surfaces::WriteOutcome>,
    auto_detected_hosts: bool,
    auto_previewed_due_to_missing_hosts: bool,
}

struct MaterializeRequest<'a> {
    command: &'a str,
    depth: usize,
    allow_self: bool,
    allow_low_confidence: bool,
    install_paths: &'a InstallPaths,
    skills_path: &'a std::path::Path,
    selection: &'a OnboardingSelection,
}

#[derive(Default, Clone, Copy)]
struct GoldenPathAdapters;

impl GoldenPathAdapters {
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

pub(crate) struct GoldenPathApp {
    route: GoldenPathRoute,
    adapters: GoldenPathAdapters,
}

impl GoldenPathApp {
    pub(crate) fn current() -> Self {
        Self {
            route: GoldenPathRoute::current(),
            adapters: GoldenPathAdapters,
        }
    }

    pub(crate) fn run_add(&self, request: AddRequest) -> Result<CommandOutcome> {
        match self.route {
            GoldenPathRoute::Legacy => self.run_add_legacy(request),
            GoldenPathRoute::Core => self.run_add_core(request),
        }
    }

    pub(crate) fn run_setup(&self, request: SetupRequest) -> Result<CommandOutcome> {
        match self.route {
            GoldenPathRoute::Legacy => self.run_setup_legacy(request),
            GoldenPathRoute::Core => self.run_setup_core(request),
        }
    }

    pub(crate) async fn run_status(&self, request: StatusRequest) -> Result<CommandOutcome> {
        super::status::StatusService::new()
            .run(self.route, request)
            .await
    }

    pub(crate) fn run_sync(&self, request: SyncRequest) -> Result<CommandOutcome> {
        super::sync::SyncService::new().run(self.route, request)
    }

    fn run_add_core(&self, request: AddRequest) -> Result<CommandOutcome> {
        let render_format = explicit_structured_format(request.format, request.pretty);
        let selection =
            self.adapters
                .select_hosts(&request.install_paths, &request.hosts, request.preview);

        emit_onboarding_header(
            render_format,
            &selection,
            &request.install_paths,
            None,
        );

        let onboarded = self.adapters.inspect_and_materialize(MaterializeRequest {
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

    fn run_add_legacy(&self, request: AddRequest) -> Result<CommandOutcome> {
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

    fn run_setup_core(&self, request: SetupRequest) -> Result<CommandOutcome> {
        let auto_detected_tools = request.tools.is_empty();
        let tools = if auto_detected_tools {
            detect_setup_tools(request.limit)
        } else {
            request.tools.clone()
        };
        if tools.is_empty() {
            return Err(sxmc::error::SxmcError::Other(
                "No CLI tools were selected or auto-detected. Re-run with `--tool <name>` or install one of the common tools Sumac scans for.".into(),
            ));
        }

        let render_format = explicit_structured_format(request.format, request.pretty);
        let selection =
            self.adapters
                .select_hosts(&request.install_paths, &request.hosts, request.preview);

        if render_format.is_none() {
            println!("Selected tools: {}", tools.join(", "));
        }
        emit_onboarding_header(
            render_format,
            &selection,
            &request.install_paths,
            Some(&tools),
        );

        let mut tool_results = Vec::new();
        for command in &tools {
            if render_format.is_none() {
                println!("Onboarding tool: {}", command);
            }
            let onboarded = self.adapters.inspect_and_materialize(MaterializeRequest {
                command,
                depth: request.depth,
                allow_self: request.allow_self,
                allow_low_confidence: request.allow_low_confidence,
                install_paths: &request.install_paths,
                skills_path: &request.skills_path,
                selection: &selection,
            })?;

            if render_format.is_none() && selection.apply {
                print_write_outcomes(&onboarded.outcomes);
            } else if render_format.is_none() {
                print_preview_outcomes(&onboarded.outcomes);
            }

            tool_results.push(json!({
                "tool": command,
                "profile": crate::profile_summary_value(&onboarded.profile),
                "outcomes": crate::write_outcomes_value(&onboarded.outcomes),
                "outcome_summary": crate::write_outcome_summary_value(&onboarded.outcomes),
            }));
        }

        if let Some(format) = render_format {
            let value = setup_result_value(SetupResultContext {
                install_paths: &request.install_paths,
                tools: &tools,
                tool_results: &tool_results,
                auto_detected_tools,
                hosts: &selection.selected_hosts,
                auto_detected_hosts: selection.auto_detected_hosts,
                preview_requested: request.preview,
                auto_previewed_due_to_missing_hosts: selection.auto_previewed_due_to_missing_hosts,
            });
            println!("{}", output::format_structured_value(&value, format));
        }

        Ok(CommandOutcome::default())
    }

    fn run_setup_legacy(&self, request: SetupRequest) -> Result<CommandOutcome> {
        let auto_detected_tools = request.tools.is_empty();
        let tools = if auto_detected_tools {
            detect_setup_tools(request.limit)
        } else {
            request.tools.clone()
        };
        if tools.is_empty() {
            return Err(sxmc::error::SxmcError::Other(
                "No CLI tools were selected or auto-detected. Re-run with `--tool <name>` or install one of the common tools Sumac scans for.".into(),
            ));
        }
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

        if render_format.is_none() {
            println!("Selected tools: {}", tools.join(", "));
        }
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

        let mut tool_results = Vec::new();
        for command in &tools {
            if render_format.is_none() {
                println!("Onboarding tool: {}", command);
            }
            let profile =
                cli_surfaces::inspect_cli_with_depth(command, request.allow_self, request.depth)?;
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

            if render_format.is_none() && apply {
                print_write_outcomes(&outcomes);
            } else if render_format.is_none() {
                print_preview_outcomes(&outcomes);
            }

            tool_results.push(json!({
                "tool": command,
                "profile": crate::profile_summary_value(&profile),
                "outcomes": crate::write_outcomes_value(&outcomes),
                "outcome_summary": crate::write_outcome_summary_value(&outcomes),
            }));
        }

        if let Some(format) = render_format {
            let value = setup_result_value(SetupResultContext {
                install_paths: &request.install_paths,
                tools: &tools,
                tool_results: &tool_results,
                auto_detected_tools,
                hosts: &selected_hosts,
                auto_detected_hosts,
                preview_requested: request.preview,
                auto_previewed_due_to_missing_hosts,
            });
            println!("{}", output::format_structured_value(&value, format));
        }

        Ok(CommandOutcome::default())
    }

}

fn emit_onboarding_header(
    render_format: Option<output::StructuredOutputFormat>,
    selection: &OnboardingSelection,
    install_paths: &InstallPaths,
    tools: Option<&[String]>,
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
        if let Some(tools) = tools {
            println!(
                "Tip: install a supported host runtime or pass --host <name> to apply directly."
            );
            let _ = tools;
        } else {
            println!(
                "Tip: install a supported host runtime or pass --host <name> to apply directly."
            );
        }
    }
}
