use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SkillFrontmatter {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub argument_hint: Option<String>,
    pub allowed_tools: Option<Vec<String>>,
    #[serde(default = "default_true")]
    pub user_invocable: bool,
    pub model: Option<String>,
    #[serde(default)]
    pub disable_model_invocation: bool,
    pub context: Option<String>,
    pub agent: Option<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone)]
pub struct SkillScript {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SkillReference {
    pub name: String,
    pub path: PathBuf,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillAssetKind {
    SkillFile,
    Script,
    Reference,
}

#[derive(Debug, Clone)]
pub struct SkillAsset {
    pub relative_path: String,
    pub path: PathBuf,
    pub kind: SkillAssetKind,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub base_dir: PathBuf,
    pub frontmatter: SkillFrontmatter,
    pub body: String,
    pub assets: Vec<SkillAsset>,
    pub scripts: Vec<SkillScript>,
    pub references: Vec<SkillReference>,
    pub source: String,
}

/// Parsed argument from argument-hint field
#[derive(Debug, Clone)]
pub struct SkillArgument {
    pub name: String,
    pub required: bool,
    pub description: String,
}
