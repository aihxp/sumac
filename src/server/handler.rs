use std::collections::HashMap;
use std::path::Component;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rmcp::model::*;
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};

use crate::discovery_snapshots::{DiscoveryGeneratedTool, DiscoveryResource};
use crate::executor;
use crate::skills::models::{Skill, SkillAsset, SkillAssetKind};
use crate::skills::parser::parse_argument_hint;

const TOOL_GET_AVAILABLE_SKILLS: &str = "get_available_skills";
const TOOL_GET_SKILL_DETAILS: &str = "get_skill_details";
const TOOL_GET_SKILL_RELATED_FILE: &str = "get_skill_related_file";

#[derive(Clone)]
pub struct SkillsServer {
    skills: Vec<Skill>,
    skill_index: HashMap<String, usize>,
    tool_index: HashMap<String, (usize, usize)>,
    discovery_tool_index: HashMap<String, usize>,
    resource_index: HashMap<String, ResourceLookup>,
    discovery_resources: Vec<DiscoveryResource>,
    discovery_tools: Vec<DiscoveryGeneratedTool>,
}

#[derive(Clone)]
enum ResourceLookup {
    SkillReference(usize, usize),
    DiscoverySnapshot(usize),
    DiscoveryIndex,
}

impl SkillsServer {
    pub fn new(skills: Vec<Skill>) -> Self {
        Self::new_with_resources(skills, Vec::new(), Vec::new())
    }

    pub fn new_with_resources(
        skills: Vec<Skill>,
        discovery_resources: Vec<DiscoveryResource>,
        discovery_tools: Vec<DiscoveryGeneratedTool>,
    ) -> Self {
        let mut skill_index = HashMap::new();
        let mut tool_index = HashMap::new();
        let mut discovery_tool_index = HashMap::new();
        let mut resource_index = HashMap::new();

        for (si, skill) in skills.iter().enumerate() {
            skill_index.insert(skill.name.clone(), si);

            for (sci, script) in skill.scripts.iter().enumerate() {
                let tool_name = Self::make_tool_name(&skill.name, &script.name);
                tool_index.insert(tool_name, (si, sci));
            }

            for (ri, reference) in skill.references.iter().enumerate() {
                resource_index.insert(
                    reference.uri.clone(),
                    ResourceLookup::SkillReference(si, ri),
                );
            }
        }

        for (index, tool) in discovery_tools.iter().enumerate() {
            discovery_tool_index.insert(tool.name.clone(), index);
        }

        if !discovery_resources.is_empty() {
            resource_index.insert(
                "sxmc-discovery://snapshots".into(),
                ResourceLookup::DiscoveryIndex,
            );
        }
        for (index, resource) in discovery_resources.iter().enumerate() {
            resource_index.insert(
                resource.uri.clone(),
                ResourceLookup::DiscoverySnapshot(index),
            );
        }

        Self {
            skills,
            skill_index,
            tool_index,
            discovery_tool_index,
            resource_index,
            discovery_resources,
            discovery_tools,
        }
    }

    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn discovery_resources(&self) -> &[DiscoveryResource] {
        &self.discovery_resources
    }

    pub fn discovery_tools(&self) -> &[DiscoveryGeneratedTool] {
        &self.discovery_tools
    }

    fn make_tool_name(skill_name: &str, script_name: &str) -> String {
        let stem = script_name
            .rsplit_once('.')
            .map(|(s, _)| s)
            .unwrap_or(script_name);
        format!(
            "{}__{}",
            skill_name.replace('-', "_"),
            stem.replace('-', "_"),
        )
    }

    fn build_script_input_schema() -> Arc<JsonObject> {
        let mut props = serde_json::Map::new();
        let mut args_obj = serde_json::Map::new();
        args_obj.insert("type".into(), "string".into());
        args_obj.insert(
            "description".into(),
            "Arguments to pass to the script".into(),
        );
        props.insert("args".into(), serde_json::Value::Object(args_obj));

        let mut schema = serde_json::Map::new();
        schema.insert("type".into(), "object".into());
        schema.insert("properties".into(), serde_json::Value::Object(props));
        Arc::new(schema)
    }

    fn build_available_skills_schema() -> Arc<JsonObject> {
        let mut schema = serde_json::Map::new();
        schema.insert("type".into(), "object".into());
        schema.insert(
            "properties".into(),
            serde_json::Value::Object(serde_json::Map::new()),
        );
        Arc::new(schema)
    }

    fn build_skill_details_schema() -> Arc<JsonObject> {
        let mut props = serde_json::Map::new();
        props.insert("name".into(), string_property("Skill name to inspect"));
        props.insert(
            "return_type".into(),
            string_property("One of: content, file_path, both"),
        );

        object_schema(props, &["name"])
    }

    fn build_skill_related_file_schema() -> Arc<JsonObject> {
        let mut props = serde_json::Map::new();
        props.insert(
            "skill_name".into(),
            string_property("Skill name containing the file"),
        );
        props.insert(
            "relative_path".into(),
            string_property("Path relative to the skill directory"),
        );
        props.insert(
            "return_type".into(),
            string_property("One of: content, file_path, both"),
        );

        object_schema(props, &["skill_name", "relative_path"])
    }

    fn hybrid_tools() -> Vec<Tool> {
        vec![
            Tool::new(
                TOOL_GET_AVAILABLE_SKILLS.to_string(),
                "List available skills with their prompt, tool, and resource metadata".to_string(),
                Self::build_available_skills_schema(),
            ),
            Tool::new(
                TOOL_GET_SKILL_DETAILS.to_string(),
                "Get detailed information for a skill, including its prompt body and file listing"
                    .to_string(),
                Self::build_skill_details_schema(),
            ),
            Tool::new(
                TOOL_GET_SKILL_RELATED_FILE.to_string(),
                "Read a file from within a skill directory using a safe relative path".to_string(),
                Self::build_skill_related_file_schema(),
            ),
        ]
    }

    fn list_available_skills(&self) -> serde_json::Value {
        serde_json::Value::Array(
            self.skills
                .iter()
                .map(|skill| {
                    serde_json::json!({
                        "name": skill.name,
                        "description": skill.frontmatter.description,
                        "prompt_name": skill.name,
                        "tools": skill
                            .scripts
                            .iter()
                            .map(|script| Self::make_tool_name(&skill.name, &script.name))
                            .collect::<Vec<_>>(),
                        "resources": skill
                            .references
                            .iter()
                            .map(|reference| reference.uri.clone())
                            .collect::<Vec<_>>(),
                    })
                })
                .collect(),
        )
    }

    fn get_skill_details(
        &self,
        args: Option<&serde_json::Map<String, serde_json::Value>>,
    ) -> Result<CallToolResult, McpError> {
        let skill_name = required_string_arg(args, "name")?;
        let return_type = return_type_arg(args)?;

        let skill = self
            .skill_index
            .get(skill_name)
            .map(|idx| &self.skills[*idx])
            .ok_or_else(|| {
                McpError::invalid_params(format!("Unknown skill: {}", skill_name), None)
            })?;

        let skill_md = skill.base_dir.join("SKILL.md");
        let file_path = skill_md.display().to_string();
        let content = std::fs::read_to_string(&skill_md).map_err(|e| {
            McpError::internal_error(
                format!("Failed to read {}: {}", skill_md.display(), e),
                None,
            )
        })?;

        match return_type {
            ReturnType::Content => Ok(CallToolResult::success(vec![Content::text(content)])),
            ReturnType::FilePath => Ok(CallToolResult::success(vec![Content::text(file_path)])),
            ReturnType::Both => json_success(serde_json::json!({
                "name": skill.name,
                "description": skill.frontmatter.description,
                "prompt_name": skill.name,
                "skill_path": file_path,
                "skill_content": content,
                "tools": skill
                    .scripts
                    .iter()
                    .map(|script| Self::make_tool_name(&skill.name, &script.name))
                    .collect::<Vec<_>>(),
                "resources": skill
                    .references
                    .iter()
                    .map(|reference| reference.uri.clone())
                    .collect::<Vec<_>>(),
                "files": list_skill_files(skill)?,
            })),
        }
    }

    fn get_skill_related_file(
        &self,
        args: Option<&serde_json::Map<String, serde_json::Value>>,
    ) -> Result<CallToolResult, McpError> {
        let skill_name = required_string_arg(args, "skill_name")?;
        let relative_path = required_string_arg(args, "relative_path")?;
        let return_type = return_type_arg(args)?;

        let skill = self
            .skill_index
            .get(skill_name)
            .map(|idx| &self.skills[*idx])
            .ok_or_else(|| {
                McpError::invalid_params(format!("Unknown skill: {}", skill_name), None)
            })?;

        let resolved_path = resolve_skill_file_path(skill, relative_path)?;
        let file_path = resolved_path.display().to_string();

        match return_type {
            ReturnType::Content => {
                let content = read_text_file(&resolved_path)?;
                Ok(CallToolResult::success(vec![Content::text(content)]))
            }
            ReturnType::FilePath => Ok(CallToolResult::success(vec![Content::text(file_path)])),
            ReturnType::Both => {
                let content = read_text_file(&resolved_path)?;
                json_success(serde_json::json!({
                    "skill_name": skill.name,
                    "relative_path": relative_path,
                    "file_path": file_path,
                    "mime_type": mime_from_name(relative_path),
                    "content": content,
                }))
            }
        }
    }
}

impl ServerHandler for SkillsServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .enable_resources()
                .build(),
        )
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let schema = Self::build_script_input_schema();
        let mut tools = Self::hybrid_tools();

        for skill in &self.skills {
            for script in &skill.scripts {
                let tool_name = Self::make_tool_name(&skill.name, &script.name);
                let tool = Tool::new(
                    tool_name,
                    format!("Run {} from skill '{}'", script.name, skill.name),
                    schema.clone(),
                );
                tools.push(tool);
            }
        }

        let discovery_schema = Self::build_available_skills_schema();
        for tool in &self.discovery_tools {
            tools.push(Tool::new(
                tool.name.clone(),
                tool.description.clone(),
                discovery_schema.clone(),
            ));
        }

        Ok(ListToolsResult {
            tools,
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let tool_name: &str = request.name.as_ref();

        match tool_name {
            TOOL_GET_AVAILABLE_SKILLS => return json_success(self.list_available_skills()),
            TOOL_GET_SKILL_DETAILS => return self.get_skill_details(request.arguments.as_ref()),
            TOOL_GET_SKILL_RELATED_FILE => {
                return self.get_skill_related_file(request.arguments.as_ref())
            }
            _ => {}
        }

        if let Some(index) = self.discovery_tool_index.get(tool_name) {
            return json_success(self.discovery_tools[*index].content.clone());
        }

        let (si, sci) = self.tool_index.get(tool_name).ok_or_else(|| {
            McpError::invalid_params(format!("Unknown tool: {}", tool_name), None)
        })?;

        let skill = &self.skills[*si];
        let script = &skill.scripts[*sci];

        let args_str = request
            .arguments
            .as_ref()
            .and_then(|args| args.get("args"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let args: Vec<&str> = if args_str.is_empty() {
            vec![]
        } else {
            args_str.split_whitespace().collect()
        };

        match executor::execute_script(&script.path, &args, &skill.base_dir, 30).await {
            Ok(result) => {
                let mut output = result.stdout;
                if result.exit_code != 0 {
                    output.push_str(&format!(
                        "\nSTDERR: {}\nExit code: {}",
                        result.stderr, result.exit_code
                    ));
                    return Ok(CallToolResult::error(vec![Content::text(output)]));
                }
                Ok(CallToolResult::success(vec![Content::text(output)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        let prompts: Vec<Prompt> = self
            .skills
            .iter()
            .map(|skill| {
                let args = skill
                    .frontmatter
                    .argument_hint
                    .as_deref()
                    .map(parse_argument_hint)
                    .unwrap_or_default();

                let prompt_args: Vec<PromptArgument> = args
                    .iter()
                    .map(|a| {
                        PromptArgument::new(a.name.clone())
                            .with_description(a.description.clone())
                            .with_required(a.required)
                    })
                    .collect();

                Prompt::new(
                    skill.name.clone(),
                    Some(skill.frontmatter.description.clone()),
                    Some(prompt_args),
                )
            })
            .collect();

        Ok(ListPromptsResult {
            prompts,
            next_cursor: None,
            meta: None,
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        let skill_idx = self.skill_index.get(&request.name).ok_or_else(|| {
            McpError::invalid_params(format!("Unknown prompt: {}", request.name), None)
        })?;

        let skill = &self.skills[*skill_idx];
        let mut body = skill.body.clone();

        if let Some(ref args) = request.arguments {
            if let Some(full_args) = args.get("arguments").and_then(|v| v.as_str()) {
                body = body.replace("$ARGUMENTS", full_args);
            }
            for (key, value) in args {
                if let Some(val_str) = value.as_str() {
                    let placeholder = format!("${}", key.to_uppercase().replace('-', "_"));
                    body = body.replace(&placeholder, val_str);
                }
            }
        }

        body = body.replace("$ARGUMENTS", "");

        let messages = vec![PromptMessage::new_text(PromptMessageRole::User, body)];

        Ok(GetPromptResult::new(messages).with_description(skill.frontmatter.description.clone()))
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let resources: Vec<Resource> = self
            .skills
            .iter()
            .flat_map(|skill| {
                skill.references.iter().map(|r| {
                    let raw = RawResource::new(r.uri.clone(), r.name.clone())
                        .with_description(format!("Reference from skill '{}'", skill.name))
                        .with_mime_type(mime_from_name(&r.name));
                    Annotated::new(raw, None)
                })
            })
            .chain(self.discovery_resources.iter().map(|resource| {
                let raw = RawResource::new(resource.uri.clone(), resource.name.clone())
                    .with_description(resource.description.clone())
                    .with_mime_type(resource.mime_type.clone());
                Annotated::new(raw, None)
            }))
            .chain((!self.discovery_resources.is_empty()).then(|| {
                let raw = RawResource::new(
                    "sxmc-discovery://snapshots".to_string(),
                    "Mounted discovery snapshot index".to_string(),
                )
                .with_description("Index of discovery snapshots exposed through this MCP server.")
                .with_mime_type("application/json");
                Annotated::new(raw, None)
            }))
            .collect();

        Ok(ListResourcesResult {
            resources,
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let uri_str = request.uri.as_str();

        let lookup = self.resource_index.get(uri_str).ok_or_else(|| {
            McpError::invalid_params(format!("Unknown resource: {}", uri_str), None)
        })?;

        match lookup {
            ResourceLookup::SkillReference(si, ri) => {
                let reference = &self.skills[*si].references[*ri];
                let content = std::fs::read_to_string(&reference.path).map_err(|e| {
                    McpError::internal_error(
                        format!("Failed to read {}: {}", reference.path.display(), e),
                        None,
                    )
                })?;

                Ok(ReadResourceResult::new(vec![ResourceContents::text(
                    content, uri_str,
                )]))
            }
            ResourceLookup::DiscoverySnapshot(index) => {
                let resource = &self.discovery_resources[*index];
                Ok(ReadResourceResult::new(vec![ResourceContents::text(
                    resource.content.clone(),
                    uri_str,
                )]))
            }
            ResourceLookup::DiscoveryIndex => {
                let content = serde_json::to_string_pretty(&serde_json::json!({
                    "count": self.discovery_resources.len(),
                    "entries": self.discovery_resources.iter().map(|resource| {
                        serde_json::json!({
                            "uri": resource.uri,
                            "name": resource.name,
                            "description": resource.description,
                            "path": resource.path.display().to_string(),
                            "source_type": resource.source_type,
                        })
                    }).collect::<Vec<_>>(),
                }))
                .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                Ok(ReadResourceResult::new(vec![ResourceContents::text(
                    content, uri_str,
                )]))
            }
        }
    }
}

fn mime_from_name(name: &str) -> String {
    match name.rsplit('.').next() {
        Some("md") => "text/markdown".to_string(),
        Some("json") => "application/json".to_string(),
        Some("yaml" | "yml") => "text/yaml".to_string(),
        Some("txt") => "text/plain".to_string(),
        Some("sh") => "text/x-shellscript".to_string(),
        Some("py") => "text/x-python".to_string(),
        _ => "text/plain".to_string(),
    }
}

#[derive(Clone, Copy)]
enum ReturnType {
    Content,
    FilePath,
    Both,
}

fn string_property(description: &str) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    obj.insert("type".into(), "string".into());
    obj.insert("description".into(), description.into());
    serde_json::Value::Object(obj)
}

fn object_schema(
    props: serde_json::Map<String, serde_json::Value>,
    required: &[&str],
) -> Arc<JsonObject> {
    let mut schema = serde_json::Map::new();
    schema.insert("type".into(), "object".into());
    schema.insert("properties".into(), serde_json::Value::Object(props));
    if !required.is_empty() {
        schema.insert(
            "required".into(),
            serde_json::Value::Array(
                required
                    .iter()
                    .map(|name| serde_json::Value::String((*name).to_string()))
                    .collect(),
            ),
        );
    }
    Arc::new(schema)
}

fn required_string_arg<'a>(
    args: Option<&'a serde_json::Map<String, serde_json::Value>>,
    key: &str,
) -> Result<&'a str, McpError> {
    args.and_then(|args| args.get(key))
        .and_then(|value| value.as_str())
        .ok_or_else(|| {
            McpError::invalid_params(format!("Missing required argument: {}", key), None)
        })
}

fn return_type_arg(
    args: Option<&serde_json::Map<String, serde_json::Value>>,
) -> Result<ReturnType, McpError> {
    match args
        .and_then(|args| args.get("return_type"))
        .and_then(|value| value.as_str())
        .unwrap_or("both")
    {
        "content" => Ok(ReturnType::Content),
        "file_path" => Ok(ReturnType::FilePath),
        "both" => Ok(ReturnType::Both),
        other => Err(McpError::invalid_params(
            format!(
                "Invalid return_type '{}'. Use one of: content, file_path, both",
                other
            ),
            None,
        )),
    }
}

fn json_success(value: serde_json::Value) -> Result<CallToolResult, McpError> {
    let text = serde_json::to_string_pretty(&value).map_err(|e| {
        McpError::internal_error(format!("Failed to serialize tool response: {}", e), None)
    })?;
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

fn read_text_file(path: &Path) -> Result<String, McpError> {
    std::fs::read_to_string(path).map_err(|e| {
        McpError::internal_error(format!("Failed to read {}: {}", path.display(), e), None)
    })
}

fn resolve_skill_file_path(skill: &Skill, relative_path: &str) -> Result<PathBuf, McpError> {
    let normalized = normalize_relative_path(relative_path)?;
    let base = skill.base_dir.canonicalize().map_err(|e| {
        McpError::internal_error(
            format!(
                "Failed to resolve skill directory {}: {}",
                skill.base_dir.display(),
                e
            ),
            None,
        )
    })?;

    let asset = managed_asset_by_relative_path(skill, &normalized).ok_or_else(|| {
        McpError::invalid_params(
            format!("Path '{}' is not a managed skill asset", relative_path),
            None,
        )
    })?;

    let resolved = asset.path.canonicalize().map_err(|e| {
        McpError::invalid_params(
            format!("Managed asset '{}' not found: {}", normalized, e),
            None,
        )
    })?;

    if !resolved.starts_with(&base) {
        return Err(McpError::invalid_params(
            format!("Managed asset '{}' escapes the skill directory", normalized),
            None,
        ));
    }

    if !resolved.is_file() {
        return Err(McpError::invalid_params(
            format!("Managed asset '{}' is not a file", normalized),
            None,
        ));
    }

    Ok(resolved)
}

fn normalize_relative_path(relative_path: &str) -> Result<String, McpError> {
    let relative = Path::new(relative_path);
    if relative.as_os_str().is_empty() || relative.is_absolute() {
        return Err(McpError::invalid_params(
            "relative_path must be a non-empty relative path".to_string(),
            None,
        ));
    }

    let mut parts = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(McpError::invalid_params(
                    format!("Path '{}' escapes the skill directory", relative_path),
                    None,
                ))
            }
            Component::Prefix(_) | Component::RootDir => {
                return Err(McpError::invalid_params(
                    "relative_path must be a non-empty relative path".to_string(),
                    None,
                ))
            }
        }
    }

    if parts.is_empty() {
        return Err(McpError::invalid_params(
            "relative_path must be a non-empty relative path".to_string(),
            None,
        ));
    }

    Ok(parts.join("/"))
}

fn managed_asset_by_relative_path<'a>(skill: &'a Skill, relative_path: &str) -> Option<&'a SkillAsset> {
    skill.assets.iter().find(|asset| asset.relative_path == relative_path)
}

fn list_skill_files(skill: &Skill) -> Result<Vec<String>, McpError> {
    let mut files = skill
        .assets
        .iter()
        .filter(|asset| matches!(
            asset.kind,
            SkillAssetKind::SkillFile | SkillAssetKind::Script | SkillAssetKind::Reference
        ))
        .map(|asset| asset.relative_path.clone())
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::models::{Skill, SkillAsset, SkillAssetKind, SkillFrontmatter};

    fn make_skill(base_dir: PathBuf) -> Skill {
        let skill_file = base_dir.join("SKILL.md");
        Skill {
            name: "test-skill".to_string(),
            base_dir,
            frontmatter: SkillFrontmatter {
                name: "test-skill".to_string(),
                description: "Test skill".to_string(),
                argument_hint: None,
                allowed_tools: None,
                user_invocable: true,
                model: None,
                disable_model_invocation: false,
                context: None,
                agent: None,
            },
            body: "Body".to_string(),
            assets: vec![SkillAsset {
                relative_path: "SKILL.md".to_string(),
                path: skill_file,
                kind: SkillAssetKind::SkillFile,
            }],
            scripts: vec![],
            references: vec![],
            source: "test".to_string(),
        }
    }

    fn make_asset(path: PathBuf, relative_path: &str, kind: SkillAssetKind) -> SkillAsset {
        SkillAsset {
            relative_path: relative_path.to_string(),
            path,
            kind,
        }
    }

    #[test]
    fn test_resolve_skill_file_path_rejects_escape() {
        let dir = tempfile::tempdir().unwrap();
        let skill_dir = dir.path().join("skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), "content").unwrap();

        let skill = make_skill(skill_dir);
        let err = resolve_skill_file_path(&skill, "../outside.txt").unwrap_err();
        assert!(
            err.message.contains("escapes the skill directory")
                || err.message.contains("not found")
        );
    }

    #[test]
    fn test_resolve_skill_file_path_rejects_unmanaged_file() {
        let dir = tempfile::tempdir().unwrap();
        let skill_dir = dir.path().join("skill");
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), "content").unwrap();
        std::fs::write(skill_dir.join("secret.txt"), "not managed").unwrap();

        let skill = make_skill(skill_dir);
        let err = resolve_skill_file_path(&skill, "secret.txt").unwrap_err();
        assert!(err.message.contains("not a managed skill asset"));
    }

    #[test]
    fn test_resolve_skill_file_path_allows_managed_nested_asset() {
        let dir = tempfile::tempdir().unwrap();
        let skill_dir = dir.path().join("skill");
        std::fs::create_dir_all(skill_dir.join("references/nested")).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), "content").unwrap();
        let guide_path = skill_dir.join("references/nested/guide.md");
        std::fs::write(&guide_path, "# Guide").unwrap();

        let mut skill = make_skill(skill_dir);
        skill.assets.push(make_asset(
            guide_path.clone(),
            "references/nested/guide.md",
            SkillAssetKind::Reference,
        ));

        let resolved = resolve_skill_file_path(&skill, "references/nested/guide.md").unwrap();
        assert_eq!(resolved, guide_path.canonicalize().unwrap());
    }

    #[test]
    fn test_list_skill_files_returns_only_managed_assets() {
        let dir = tempfile::tempdir().unwrap();
        let skill_dir = dir.path().join("skill");
        std::fs::create_dir_all(skill_dir.join("references")).unwrap();
        std::fs::create_dir_all(skill_dir.join("scripts")).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), "content").unwrap();
        let guide_path = skill_dir.join("references/guide.md");
        let script_path = skill_dir.join("scripts/run.sh");
        std::fs::write(&guide_path, "# Guide").unwrap();
        std::fs::write(&script_path, "#!/bin/bash").unwrap();
        std::fs::write(skill_dir.join("secret.txt"), "not managed").unwrap();

        let mut skill = make_skill(skill_dir);
        skill.assets.push(make_asset(
            script_path,
            "scripts/run.sh",
            SkillAssetKind::Script,
        ));
        skill.assets.push(make_asset(
            guide_path,
            "references/guide.md",
            SkillAssetKind::Reference,
        ));

        let files = list_skill_files(&skill).unwrap();
        assert_eq!(
            files,
            vec![
                "SKILL.md".to_string(),
                "references/guide.md".to_string(),
                "scripts/run.sh".to_string(),
            ]
        );
    }
}
