use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    #[serde(default)]
    pub data_paths: Vec<PathBuf>,
    #[serde(default)]
    pub enum_paths: Vec<PathBuf>,
    #[serde(default = "default_output")]
    pub output_directory: PathBuf,
    #[serde(default = "default_true")]
    pub clear_output_directory_before_write: bool,
    #[serde(default = "default_true")]
    pub phpdoc_overrides_types: bool,
    #[serde(default = "default_plugins")]
    pub plugins: Vec<String>,
    /// Declarative FQCN → TypeScript mappings (app plugins like UploadedFile).
    #[serde(default)]
    pub type_maps: Vec<TypeMap>,
}

/// Map a PHP class FQCN to TypeScript snippets, optionally emitting a helper file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeMap {
    #[serde(rename = "match", deserialize_with = "string_or_vec")]
    pub match_fqcns: Vec<String>,
    pub interface: String,
    #[serde(default)]
    pub encoded: Option<String>,
    pub schema: String,
    #[serde(default, rename = "enum")]
    pub enum_type: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub file_content: Option<String>,
    /// `.ts` file to read; relative paths resolve against the config file directory.
    #[serde(default)]
    pub file: Option<PathBuf>,
}

fn string_or_vec<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }
    match OneOrMany::deserialize(deserializer)? {
        OneOrMany::One(s) => Ok(vec![s]),
        OneOrMany::Many(v) => Ok(v),
    }
}

fn default_output() -> PathBuf {
    PathBuf::from("resources/js/schemas")
}

fn default_true() -> bool {
    true
}

fn default_plugins() -> Vec<String> {
    vec![
        "length_aware_paginator".into(),
        "lazy_optional".into(),
        "hidden".into(),
        "snake_case_attribute".into(),
        "date".into(),
        "collection".into(),
        "type_enum".into(),
        "effect_schema_enum".into(),
        "const_object_enum".into(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_paths: Vec::new(),
            enum_paths: Vec::new(),
            output_directory: default_output(),
            clear_output_directory_before_write: true,
            phpdoc_overrides_types: true,
            plugins: default_plugins(),
            type_maps: Vec::new(),
        }
    }
}

impl Config {
    pub fn from_toml_str(s: &str) -> crate::Result<Self> {
        toml::from_str(s).map_err(|e| crate::Error::Config(e.to_string()))
    }

    pub fn from_json_str(s: &str) -> crate::Result<Self> {
        serde_json::from_str(s).map_err(|e| crate::Error::Config(e.to_string()))
    }

    pub fn load_file(path: &std::path::Path) -> crate::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let mut cfg = if path.extension().and_then(|e| e.to_str()) == Some("json") {
            Self::from_json_str(&text)?
        } else {
            Self::from_toml_str(&text)?
        };
        let base = path
            .parent()
            .and_then(|parent| parent.canonicalize().ok())
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        cfg.resolve_relative_paths(&base)?;
        Ok(cfg)
    }

    fn resolve_relative_paths(&mut self, base: &std::path::Path) -> crate::Result<()> {
        self.data_paths = self
            .data_paths
            .iter()
            .map(|p| absolutize(base, p))
            .collect();
        self.enum_paths = self
            .enum_paths
            .iter()
            .map(|p| absolutize(base, p))
            .collect();
        self.output_directory = absolutize(base, &self.output_directory);
        for map in &mut self.type_maps {
            if map.file_content.is_some() {
                continue;
            }
            let Some(file) = &map.file else {
                continue;
            };
            let path = absolutize(base, file);
            map.file_content = Some(std::fs::read_to_string(&path)?);
        }
        Ok(())
    }
}

fn absolutize(base: &std::path::Path, path: &std::path::Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}
