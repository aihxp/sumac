use std::time::Duration;

use std::collections::HashMap;

use serde_json::{json, Value};

use crate::client::commands::CommandDef;
use crate::client::graphql;
use crate::client::openapi;
use crate::error::{Result, SxmcError};
use crate::projection::{apply_offset_limit, retain_object_fields};

#[derive(Clone, Debug, Default)]
pub struct ListSelectors<'a> {
    pub compact: bool,
    pub names_only: bool,
    pub required_only: bool,
    pub counts_only: bool,
    pub no_descriptions: bool,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub fields: Option<&'a [String]>,
}

/// The detected API type.
#[derive(Debug, Clone, PartialEq)]
pub enum ApiType {
    OpenApi,
    GraphQL,
}

/// A unified API client that auto-detects the API type and delegates.
pub enum ApiClient {
    OpenApi(openapi::OpenApiSpec),
    GraphQL(graphql::GraphQLClient),
}

impl ApiClient {
    /// Auto-detect API type from source and connect.
    pub async fn connect(
        source: &str,
        auth_headers: &[(String, String)],
        timeout: Option<Duration>,
    ) -> Result<Self> {
        let api_type = detect_api_type(source, auth_headers, timeout).await?;

        match api_type {
            ApiType::OpenApi => {
                let spec = openapi::OpenApiSpec::load(source, auth_headers, timeout).await?;
                Ok(ApiClient::OpenApi(spec))
            }
            ApiType::GraphQL => {
                let client = graphql::GraphQLClient::connect(source, auth_headers, timeout).await?;
                Ok(ApiClient::GraphQL(client))
            }
        }
    }

    /// Get commands for all operations.
    pub fn commands(&self) -> Vec<CommandDef> {
        match self {
            ApiClient::OpenApi(spec) => spec.commands(),
            ApiClient::GraphQL(client) => client.commands(),
        }
    }

    /// Execute an operation by name.
    pub async fn execute(&self, name: &str, args: &HashMap<String, String>) -> Result<Value> {
        match self {
            ApiClient::OpenApi(spec) => spec.execute(name, args).await,
            ApiClient::GraphQL(client) => client.execute(name, args).await,
        }
    }

    /// Format a listing of available operations.
    pub fn format_list(&self, search: Option<&str>, selectors: &ListSelectors<'_>) -> String {
        match self {
            ApiClient::OpenApi(spec) => {
                let ops = spec.list_operations(search);
                openapi::format_operation_list(&ops, None, selectors)
            }
            ApiClient::GraphQL(client) => {
                let ops = client.list_operations(search);
                graphql::format_graphql_list(&ops, None, selectors)
            }
        }
    }

    /// Return a structured listing of available operations.
    pub fn list_value(&self, search: Option<&str>, selectors: &ListSelectors<'_>) -> Value {
        if selectors.names_only {
            match self {
                ApiClient::OpenApi(spec) => {
                    let mut operations = spec
                        .list_operations(search)
                        .into_iter()
                        .map(|op| Value::String(op.operation_id.clone()))
                        .collect::<Vec<_>>();
                    apply_offset_limit(&mut operations, selectors.offset, selectors.limit);
                    json!({
                        "api_type": self.api_type(),
                        "search": search,
                        "compact": false,
                        "names_only": true,
                        "required_only": false,
                        "counts_only": false,
                        "offset": selectors.offset,
                        "limit": selectors.limit,
                        "count": operations.len(),
                        "operations": operations,
                    })
                }
                ApiClient::GraphQL(client) => {
                    let mut operations = client
                        .list_operations(search)
                        .into_iter()
                        .map(|op| Value::String(op.name.clone()))
                        .collect::<Vec<_>>();
                    apply_offset_limit(&mut operations, selectors.offset, selectors.limit);
                    json!({
                        "api_type": self.api_type(),
                        "search": search,
                        "compact": false,
                        "names_only": true,
                        "required_only": false,
                        "counts_only": false,
                        "offset": selectors.offset,
                        "limit": selectors.limit,
                        "count": operations.len(),
                        "operations": operations,
                    })
                }
            }
        } else if selectors.counts_only {
            let total_count = match self {
                ApiClient::OpenApi(spec) => spec.list_operations(search).len(),
                ApiClient::GraphQL(client) => client.list_operations(search).len(),
            };
            let count = selectors
                .limit
                .map(|limit| {
                    total_count
                        .saturating_sub(selectors.offset.unwrap_or(0))
                        .min(limit)
                })
                .unwrap_or_else(|| total_count.saturating_sub(selectors.offset.unwrap_or(0)));
            json!({
                "api_type": self.api_type(),
                "search": search,
                "compact": selectors.compact,
                "names_only": false,
                "required_only": selectors.required_only,
                "counts_only": true,
                "offset": selectors.offset,
                "limit": selectors.limit,
                "total_count": total_count,
                "count": count,
            })
        } else if selectors.compact || selectors.required_only {
            match self {
                ApiClient::OpenApi(spec) => {
                    let mut operations = spec
                        .list_operations(search)
                        .into_iter()
                        .map(|op| {
                            let mut value = if selectors.required_only {
                                json!({
                                    "name": op.operation_id,
                                    "required_params": op.required_param_names(),
                                    "required_param_count": op.parameters.iter().filter(|param| param.required).count()
                                        + usize::from(op.request_body_schema.is_some()),
                                })
                            } else {
                                json!({
                                    "name": op.operation_id,
                                    "method": op.method.to_uppercase(),
                                    "path": op.path,
                                    "required_params": op.required_param_names(),
                                    "required_param_count": op.parameters.iter().filter(|param| param.required).count()
                                        + usize::from(op.request_body_schema.is_some()),
                                })
                            };
                            if let Some(fields) = selectors.fields {
                                value = retain_object_fields(value, fields);
                            }
                            value
                        })
                        .collect::<Vec<_>>();
                    apply_offset_limit(&mut operations, selectors.offset, selectors.limit);
                    json!({
                        "api_type": self.api_type(),
                        "search": search,
                        "compact": selectors.compact,
                        "names_only": false,
                        "required_only": selectors.required_only,
                        "counts_only": false,
                        "offset": selectors.offset,
                        "limit": selectors.limit,
                        "count": operations.len(),
                        "operations": operations,
                    })
                }
                ApiClient::GraphQL(client) => {
                    let mut operations = client
                        .list_operations(search)
                        .into_iter()
                        .map(|op| {
                            let mut value = if selectors.required_only {
                                json!({
                                    "name": op.name,
                                    "required_args": op.required_arg_names(),
                                    "required_arg_count": op.args.iter().filter(|arg| arg.required).count(),
                                })
                            } else {
                                json!({
                                    "name": op.name,
                                    "kind": op.kind_label(),
                                    "required_args": op.required_arg_names(),
                                    "required_arg_count": op.args.iter().filter(|arg| arg.required).count(),
                                })
                            };
                            if let Some(fields) = selectors.fields {
                                value = retain_object_fields(value, fields);
                            }
                            value
                        })
                        .collect::<Vec<_>>();
                    apply_offset_limit(&mut operations, selectors.offset, selectors.limit);
                    json!({
                        "api_type": self.api_type(),
                        "search": search,
                        "compact": selectors.compact,
                        "names_only": false,
                        "required_only": selectors.required_only,
                        "counts_only": false,
                        "offset": selectors.offset,
                        "limit": selectors.limit,
                        "count": operations.len(),
                        "operations": operations,
                    })
                }
            }
        } else {
            let pattern = search.map(str::to_lowercase);
            let mut commands: Vec<Value> = self
                .commands()
                .into_iter()
                .filter(|cmd| {
                    if let Some(pattern) = &pattern {
                        cmd.name.to_lowercase().contains(pattern)
                            || cmd.description.to_lowercase().contains(pattern)
                    } else {
                        true
                    }
                })
                .map(|cmd| {
                    let mut value = serde_json::to_value(cmd).unwrap_or_else(|_| json!({}));
                    if selectors.no_descriptions {
                        if let Some(object) = value.as_object_mut() {
                            object.remove("description");
                        }
                    }
                    if let Some(fields) = selectors.fields {
                        value = retain_object_fields(value, fields);
                    }
                    value
                })
                .collect();
            apply_offset_limit(&mut commands, selectors.offset, selectors.limit);

            json!({
                "api_type": self.api_type(),
                "search": search,
                "compact": false,
                "names_only": false,
                "required_only": false,
                "counts_only": false,
                "offset": selectors.offset,
                "limit": selectors.limit,
                "count": commands.len(),
                "operations": commands,
            })
        }
    }

    /// Get the API type label.
    pub fn api_type(&self) -> &str {
        match self {
            ApiClient::OpenApi(_) => "OpenAPI",
            ApiClient::GraphQL(_) => "GraphQL",
        }
    }
}

/// Detect the API type from a source URL or file path.
async fn detect_api_type(
    source: &str,
    auth_headers: &[(String, String)],
    timeout: Option<Duration>,
) -> Result<ApiType> {
    let lower = source.to_lowercase();

    // File extension hints
    if lower.ends_with(".json") || lower.ends_with(".yaml") || lower.ends_with(".yml") {
        return Ok(ApiType::OpenApi);
    }

    // URL path hints
    if lower.contains("openapi") || lower.contains("swagger") {
        return Ok(ApiType::OpenApi);
    }

    if lower.contains("graphql") || lower.contains("/gql") {
        return Ok(ApiType::GraphQL);
    }

    // If it's a URL, try to fetch and detect from content
    if source.starts_with("http://") || source.starts_with("https://") {
        return detect_from_url(source, auth_headers, timeout).await;
    }

    // If it's a file, try to detect from content
    if let Ok(content) = std::fs::read_to_string(source) {
        return detect_from_content(&content);
    }

    Err(SxmcError::Other(format!(
        "Cannot determine API type for: {}. Use --spec or --graphql to specify explicitly.",
        source
    )))
}

/// Detect API type by fetching content from a URL.
async fn detect_from_url(
    url: &str,
    auth_headers: &[(String, String)],
    timeout: Option<Duration>,
) -> Result<ApiType> {
    let client = build_client(auth_headers, timeout)?;
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| SxmcError::Other(format!("Failed to fetch: {}", e)))?;

    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    // GraphQL endpoints typically don't return JSON specs on GET
    // OpenAPI specs are served as JSON/YAML
    let text = resp
        .text()
        .await
        .map_err(|e| SxmcError::Other(format!("Failed to read response: {}", e)))?;

    if content_type.contains("json") || content_type.contains("yaml") {
        return detect_from_content(&text);
    }

    // Last resort: try parsing as OpenAPI
    detect_from_content(&text)
}

/// Detect API type from content.
fn detect_from_content(content: &str) -> Result<ApiType> {
    // Try JSON parse
    if let Ok(val) = serde_json::from_str::<Value>(content) {
        if val.get("openapi").is_some() || val.get("swagger").is_some() {
            return Ok(ApiType::OpenApi);
        }
        if val.pointer("/data/__schema").is_some() {
            return Ok(ApiType::GraphQL);
        }
    }

    // YAML indicators
    if content.contains("openapi:") || content.contains("swagger:") {
        return Ok(ApiType::OpenApi);
    }

    Err(SxmcError::Other(
        "Cannot determine API type from content. Use --spec or --graphql to specify explicitly."
            .to_string(),
    ))
}

fn build_client(
    auth_headers: &[(String, String)],
    timeout: Option<Duration>,
) -> Result<reqwest::Client> {
    let mut header_map = reqwest::header::HeaderMap::new();
    for (key, value) in auth_headers {
        if let (Ok(name), Ok(val)) = (
            key.parse::<reqwest::header::HeaderName>(),
            value.parse::<reqwest::header::HeaderValue>(),
        ) {
            header_map.insert(name, val);
        }
    }

    let mut builder = reqwest::Client::builder().default_headers(header_map);
    if let Some(timeout) = timeout {
        builder = builder.timeout(timeout);
    }

    builder
        .build()
        .map_err(|e| SxmcError::Other(format!("Failed to build HTTP client: {}", e)))
}
