use thiserror::Error;

#[derive(Error, Debug)]
pub enum SxmcError {
    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Failed to parse SKILL.md: {0}")]
    ParseError(String),

    #[error("Failed to parse frontmatter YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Script execution failed: {0}")]
    ExecutionError(String),

    #[error("Script timed out after {0} seconds")]
    TimeoutError(u64),

    #[error("MCP error: {0}")]
    McpError(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SxmcError>;
