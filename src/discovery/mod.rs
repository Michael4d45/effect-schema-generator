mod extract;

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::config::Config;
use crate::error::Result;
use extract::{extract_declarations, Declaration, DeclarationKind};

const SPATIE_DATA: &str = "Spatie\\LaravelData\\Data";

/// Discover fully-qualified Data class names under configured paths.
pub fn discover_data_classes(config: &Config) -> Result<Vec<String>> {
    let declarations = scan_declarations(&config.data_paths);
    let parents = class_parent_map(&declarations);

    let mut found = BTreeSet::new();
    for declaration in &declarations {
        if matches!(declaration.kind, DeclarationKind::Class { .. })
            && extends_spatie_data(&declaration.fqcn, &parents)
        {
            found.insert(declaration.fqcn.clone());
        }
    }
    Ok(found.into_iter().collect())
}

/// Discover fully-qualified enum names under configured paths.
pub fn discover_enums(config: &Config) -> Result<Vec<String>> {
    let mut found = BTreeSet::new();
    for declaration in scan_declarations(&config.enum_paths) {
        if matches!(declaration.kind, DeclarationKind::Enum) {
            found.insert(declaration.fqcn);
        }
    }
    Ok(found.into_iter().collect())
}

/// Scan PHP files and return FQCNs for class or enum declarations.
pub fn scan_php_files(paths: &[PathBuf]) -> Result<Vec<(PathBuf, String)>> {
    let mut pairs = BTreeSet::new();
    for declaration in scan_declarations(paths) {
        pairs.insert((declaration.path, declaration.fqcn));
    }
    Ok(pairs.into_iter().collect())
}

fn scan_declarations(paths: &[PathBuf]) -> Vec<Declaration> {
    let mut declarations = Vec::new();
    for file in php_files(paths) {
        declarations.extend(extract_declarations(&file));
    }
    declarations
}

fn php_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for path in paths {
        if !path.is_dir() {
            continue;
        }
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.into_path();
            if is_php_source(&path) {
                files.push(path);
            }
        }
    }
    files.sort();
    files.dedup();
    files
}

fn is_php_source(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    name.ends_with(".php") && !name.ends_with(".blade.php")
}

fn class_parent_map(declarations: &[Declaration]) -> HashMap<String, String> {
    declarations
        .iter()
        .filter_map(|declaration| match &declaration.kind {
            DeclarationKind::Class {
                extends: Some(parent),
            } => Some((declaration.fqcn.clone(), parent.clone())),
            _ => None,
        })
        .collect()
}

fn extends_spatie_data(fqcn: &str, parents: &HashMap<String, String>) -> bool {
    let mut visiting = HashSet::new();
    let mut current = fqcn;
    loop {
        if !visiting.insert(current.to_string()) {
            return false;
        }
        let Some(parent) = parents.get(current) else {
            return false;
        };
        if parent == SPATIE_DATA {
            return true;
        }
        current = parent;
    }
}
