use std::path::PathBuf;

use sxmc::error::Result;
use sxmc::paths::InstallPaths;
use sxmc::skills::{discovery, generator, install as skill_install};

use crate::command_handlers::{
    cmd_skills_info, cmd_skills_list, cmd_skills_run, SkillListOptions,
};

use super::CommandOutcome;

#[derive(Clone)]
pub(crate) enum SkillsRequest {
    List(SkillsListRequest),
    Info(SkillsInfoRequest),
    Run(SkillsRunRequest),
    Create(SkillsCreateRequest),
    Install(SkillsInstallRequest),
    Update(SkillsUpdateRequest),
}

#[derive(Clone)]
pub(crate) struct SkillsListRequest {
    pub(crate) paths: Option<Vec<PathBuf>>,
    pub(crate) installed: bool,
    pub(crate) install_paths: Option<InstallPaths>,
    pub(crate) skills_path: PathBuf,
    pub(crate) json: bool,
    pub(crate) names_only: bool,
    pub(crate) counts_only: bool,
    pub(crate) no_descriptions: bool,
    pub(crate) fields: Option<Vec<String>>,
    pub(crate) offset: Option<usize>,
    pub(crate) limit: Option<usize>,
}

#[derive(Clone)]
pub(crate) struct SkillsInfoRequest {
    pub(crate) paths: Option<Vec<PathBuf>>,
    pub(crate) name: String,
    pub(crate) summary_only: bool,
}

#[derive(Clone)]
pub(crate) struct SkillsRunRequest {
    pub(crate) paths: Option<Vec<PathBuf>>,
    pub(crate) script: Option<String>,
    pub(crate) env_vars: Vec<String>,
    pub(crate) print_body: bool,
    pub(crate) name: String,
    pub(crate) arguments: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct SkillsCreateRequest {
    pub(crate) source: String,
    pub(crate) output_dir: PathBuf,
    pub(crate) auth_headers: Vec<(String, String)>,
}

#[derive(Clone)]
pub(crate) struct SkillsInstallRequest {
    pub(crate) source: String,
    pub(crate) repo_subpath: Option<String>,
    pub(crate) reference: Option<String>,
    pub(crate) install_paths: InstallPaths,
    pub(crate) skills_path: PathBuf,
}

#[derive(Clone)]
pub(crate) struct SkillsUpdateRequest {
    pub(crate) name: Option<String>,
    pub(crate) install_paths: InstallPaths,
    pub(crate) skills_path: PathBuf,
}

pub(crate) struct SkillsService;

impl SkillsService {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) async fn run(&self, request: SkillsRequest) -> Result<CommandOutcome> {
        match request {
            SkillsRequest::List(request) => self.run_list(request),
            SkillsRequest::Info(request) => self.run_info(request),
            SkillsRequest::Run(request) => self.run_run(request).await,
            SkillsRequest::Create(request) => self.run_create(request).await,
            SkillsRequest::Install(request) => self.run_install(request),
            SkillsRequest::Update(request) => self.run_update(request),
        }
    }

    fn run_list(&self, request: SkillsListRequest) -> Result<CommandOutcome> {
        let paths = if request.installed {
            let install_paths = request.install_paths.as_ref().expect(
                "installed skills listing requires install paths to resolve the managed skill root",
            );
            vec![install_paths.resolve_skills_path(&request.skills_path)]
        } else {
            request.paths.unwrap_or_else(discovery::default_paths)
        };

        cmd_skills_list(
            &paths,
            SkillListOptions {
                json_output: request.json,
                installed_only: request.installed,
                names_only: request.names_only,
                counts_only: request.counts_only,
                no_descriptions: request.no_descriptions,
                fields: request.fields.as_deref(),
                offset: request.offset,
                limit: request.limit,
            },
        )?;

        Ok(CommandOutcome::default())
    }

    fn run_info(&self, request: SkillsInfoRequest) -> Result<CommandOutcome> {
        let paths = request.paths.unwrap_or_else(discovery::default_paths);
        cmd_skills_info(&paths, &request.name, request.summary_only)?;
        Ok(CommandOutcome::default())
    }

    async fn run_run(&self, request: SkillsRunRequest) -> Result<CommandOutcome> {
        let paths = request.paths.unwrap_or_else(discovery::default_paths);
        cmd_skills_run(
            &paths,
            &request.name,
            request.script.as_deref(),
            &request.env_vars,
            request.print_body,
            &request.arguments,
        )
        .await?;
        Ok(CommandOutcome::default())
    }

    async fn run_create(&self, request: SkillsCreateRequest) -> Result<CommandOutcome> {
        let skill_dir = generator::generate_from_openapi(
            &request.source,
            &request.output_dir,
            &request.auth_headers,
        )
        .await?;
        println!("Generated skill at: {}", skill_dir.display());
        Ok(CommandOutcome::default())
    }

    fn run_install(&self, request: SkillsInstallRequest) -> Result<CommandOutcome> {
        let report = skill_install::install_skill(skill_install::SkillInstallRequest {
            source: &request.source,
            repo_subpath: request.repo_subpath.as_deref(),
            reference: request.reference.as_deref(),
            install_paths: &request.install_paths,
            skills_path: &request.skills_path,
        })?;
        println!(
            "Installed skill `{}` to {} ({})",
            report.name,
            report.target_dir.display(),
            report.install_scope.as_str()
        );
        Ok(CommandOutcome::default())
    }

    fn run_update(&self, request: SkillsUpdateRequest) -> Result<CommandOutcome> {
        let reports = skill_install::update_skills(skill_install::SkillUpdateRequest {
            name: request.name.as_deref(),
            install_paths: &request.install_paths,
            skills_path: &request.skills_path,
        })?;
        for report in reports {
            println!(
                "Updated skill `{}` at {} ({})",
                report.name,
                report.target_dir.display(),
                report.install_scope.as_str()
            );
        }
        Ok(CommandOutcome::default())
    }
}
