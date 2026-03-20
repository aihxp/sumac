use rmcp::model::CallToolResult;

/// Format a CallToolResult for display.
pub fn format_tool_result(result: &CallToolResult, pretty: bool) -> String {
    let texts: Vec<String> = result
        .content
        .iter()
        .filter_map(|c| c.raw.as_text().map(|t| t.text.clone()))
        .collect();

    let output = texts.join("\n");

    if pretty {
        // Try to parse as JSON and pretty-print
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&output) {
            if let Ok(pretty_str) = serde_json::to_string_pretty(&val) {
                return pretty_str;
            }
        }
    }

    output
}

/// Format MCP tools as a list for display.
pub fn format_tool_list(tools: &[rmcp::model::Tool], search: Option<&str>) -> String {
    let mut lines = Vec::new();

    for tool in tools {
        let name = tool.name.as_ref();
        let desc = tool.description.as_deref().unwrap_or("");

        if let Some(pattern) = search {
            let pattern_lower = pattern.to_lowercase();
            if !name.to_lowercase().contains(&pattern_lower)
                && !desc.to_lowercase().contains(&pattern_lower)
            {
                continue;
            }
        }

        lines.push(format!("  {}", name));
        if !desc.is_empty() {
            lines.push(format!("    {}", desc));
        }
    }

    if lines.is_empty() {
        if search.is_some() {
            return "No matching tools found.".to_string();
        }
        return "No tools available.".to_string();
    }

    format!("Tools ({}):\n{}", tools.len(), lines.join("\n"))
}

/// Format MCP prompts as a list for display.
pub fn format_prompt_list(prompts: &[rmcp::model::Prompt]) -> String {
    let mut lines = Vec::new();

    for prompt in prompts {
        lines.push(format!("  {}", prompt.name));
        if let Some(ref desc) = prompt.description {
            lines.push(format!("    {}", desc));
        }
    }

    if lines.is_empty() {
        return "No prompts available.".to_string();
    }

    format!("Prompts ({}):\n{}", prompts.len(), lines.join("\n"))
}

/// Format MCP resources as a list for display.
pub fn format_resource_list(resources: &[rmcp::model::Resource]) -> String {
    let mut lines = Vec::new();

    for resource in resources {
        lines.push(format!("  {} ({})", resource.name, resource.uri));
        if let Some(ref desc) = resource.description {
            lines.push(format!("    {}", desc));
        }
    }

    if lines.is_empty() {
        return "No resources available.".to_string();
    }

    format!("Resources ({}):\n{}", resources.len(), lines.join("\n"))
}
