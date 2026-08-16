mod types;

pub use types::default_transform;

use crate::config::Config;
use crate::ir::{EnumIr, NamespaceIr, RootIr, SchemaIr, WriterContext};
use crate::plugins::{
    is_enum_writer, ConstObjectEnumWriter, EffectSchemaEnumWriter, Transformer, TypeEnumWriter,
};
use crate::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use types::{collect_referenced_types, transform_type, write_property_line, ReferencedType};

pub fn relative_import_path(from_file: &str, to_file: &str) -> String {
    let from_dir = std::path::Path::new(from_file)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    let to_path = std::path::Path::new(to_file);
    let to_dir = to_path
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    let to_file_stem = to_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    if from_dir == to_dir {
        return format!("./{to_file_stem}");
    }

    let from_parts: Vec<&str> = if from_dir.is_empty() || from_dir == "." {
        Vec::new()
    } else {
        from_dir.split('/').filter(|s| !s.is_empty()).collect()
    };
    let to_parts: Vec<&str> = if to_dir.is_empty() || to_dir == "." {
        Vec::new()
    } else {
        to_dir.split('/').filter(|s| !s.is_empty()).collect()
    };

    let mut common = 0;
    for (a, b) in from_parts.iter().zip(to_parts.iter()) {
        if a != b {
            break;
        }
        common += 1;
    }
    let up = from_parts.len() - common;
    let mut rel = (0..up).map(|_| "..").collect::<Vec<_>>().join("/");
    let down = to_parts[common..].join("/");
    if !down.is_empty() {
        if rel.is_empty() {
            rel = down;
        } else {
            rel = format!("{rel}/{down}");
        }
    }
    let path = if rel.is_empty() {
        format!("./{to_file_stem}")
    } else {
        format!("{rel}/{to_file_stem}")
    };
    if path.starts_with('.') {
        path
    } else {
        format!("./{path}")
    }
}

pub fn schema_file_path(namespace: &str, schema_name: &str) -> String {
    format!("{}/{}.ts", namespace.replace('\\', "/"), schema_name)
}

pub fn enum_file_path(namespace: &str, enum_name: &str) -> String {
    format!("{}/{}.ts", namespace.replace('\\', "/"), enum_name)
}

pub fn fqcn_file_path(fqcn: &str) -> String {
    let parts: Vec<&str> = fqcn.trim_start_matches('\\').split('\\').collect();
    if let Some((name, ns)) = parts.split_last() {
        if ns.is_empty() {
            format!("{name}.ts")
        } else {
            format!("{}/{name}.ts", ns.join("/"))
        }
    } else {
        format!("{fqcn}.ts")
    }
}

pub fn write(ir: &RootIr, transformers: &[Box<dyn Transformer>], config: &Config) -> Result<()> {
    let mut ir = ir.clone();
    for transformer in transformers {
        transformer.preprocess(&mut ir);
    }

    if config.clear_output_directory_before_write {
        clear_output_directory_contents(&config.output_directory)?;
    }

    write_transformer_files(transformers, &config.output_directory)?;

    let enum_writers = enum_writers(transformers);

    for ns in ir.namespaces.values() {
        for schema in &ns.schemas {
            let file_path = schema_file_path(&ns.namespace, &schema.name);
            let content = render_schema_file(&file_path, ns, schema, transformers);
            write_output_file(&config.output_directory, &file_path, &content)?;
        }
        for enum_ir in &ns.enums {
            let file_path = enum_file_path(&ns.namespace, &enum_ir.name);
            let content = render_enum_file(enum_ir, &enum_writers);
            write_output_file(&config.output_directory, &file_path, &content)?;
        }
    }

    Ok(())
}

fn enum_writers(transformers: &[Box<dyn Transformer>]) -> Vec<&dyn Transformer> {
    let found: Vec<&dyn Transformer> = transformers
        .iter()
        .map(|t| t.as_ref())
        .filter(|t| is_enum_writer(*t))
        .collect();
    found
}

fn write_transformer_files(
    transformers: &[Box<dyn Transformer>],
    output_directory: &Path,
) -> Result<()> {
    for transformer in transformers {
        if !transformer.provides_file() {
            continue;
        }
        let Some(file_path) = transformer.file_path() else {
            continue;
        };
        let Some(content) = transformer.file_content() else {
            continue;
        };
        write_output_file(output_directory, &file_path, &content)?;
    }
    Ok(())
}

fn write_output_file(output_directory: &Path, relative: &str, content: &str) -> Result<()> {
    let full = output_directory.join(relative);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = format!("{}\n", content.trim_end());
    fs::write(full, content)?;
    Ok(())
}

fn clear_output_directory_contents(dir: &Path) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn render_schema_file(
    file_path: &str,
    ns: &NamespaceIr,
    schema: &SchemaIr,
    transformers: &[Box<dyn Transformer>],
) -> String {
    let local_names: BTreeSet<String> = std::iter::once(schema.name.clone())
        .chain(ns.enums.iter().map(|e| e.name.clone()))
        .collect();

    let mut referenced: BTreeMap<String, ReferencedType> = BTreeMap::new();
    for prop in schema.properties.iter().filter(|p| !p.hidden) {
        collect_referenced_types(&prop.ty, transformers, &mut referenced);
    }

    let mut imports: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    add_imports_for_refs(
        file_path,
        &referenced,
        &local_names,
        transformers,
        &mut imports,
    );

    let interface = render_interface(schema, "", WriterContext::Interface, transformers);
    let encoded = render_interface(
        schema,
        "Encoded",
        WriterContext::EncodedInterface,
        transformers,
    );
    let schema_export = render_effect_schema(schema, transformers);

    let exports = [interface, encoded, schema_export];
    assemble_file(&imports, &exports)
}

fn render_enum_file(enum_ir: &EnumIr, writers: &[&dyn Transformer]) -> String {
    let mut exports = Vec::new();
    if writers.is_empty() {
        exports.push(TypeEnumWriter::write_enum(enum_ir));
        exports.push(EffectSchemaEnumWriter::write_enum(enum_ir));
        exports.push(ConstObjectEnumWriter::write_enum(enum_ir));
    } else {
        for writer in writers {
            let content = writer.transform(
                crate::plugins::TransformInput::Enum(enum_ir),
                WriterContext::Enum,
                &[],
                &[],
                &|_, _| String::new(),
            );
            if !content.is_empty() {
                exports.push(content);
            }
        }
    }
    assemble_file(&BTreeMap::new(), &exports)
}

fn render_interface(
    schema: &SchemaIr,
    suffix: &str,
    context: WriterContext,
    transformers: &[Box<dyn Transformer>],
) -> String {
    let mut lines = Vec::new();
    for prop in schema.properties.iter().filter(|p| !p.hidden) {
        let ty = transform_type(&prop.ty, context, transformers);
        lines.push(write_property_line(prop, &ty, context));
    }
    format!(
        "export interface {}{suffix} {{\n{}\n}}",
        schema.name,
        lines.join("\n")
    )
}

fn render_effect_schema(schema: &SchemaIr, transformers: &[Box<dyn Transformer>]) -> String {
    let mut lines = Vec::new();
    for prop in schema.properties.iter().filter(|p| !p.hidden) {
        let ty = transform_type(&prop.ty, WriterContext::Schema, transformers);
        lines.push(write_property_line(prop, &ty, WriterContext::Schema));
    }
    format!(
        "export const {name}Schema: S.Schema<{name}, {name}Encoded> = S.Struct({{\n{}\n}});",
        lines.join(",\n"),
        name = schema.name
    )
}

fn add_imports_for_refs(
    current_file: &str,
    referenced: &BTreeMap<String, ReferencedType>,
    local_names: &BTreeSet<String>,
    transformers: &[Box<dyn Transformer>],
    imports: &mut BTreeMap<String, BTreeSet<String>>,
) {
    for info in referenced.values() {
        if local_names.contains(&info.alias) {
            continue;
        }
        let target = if let Some(plugin_path) = transformer_file_path_for(&info.fqcn, transformers)
        {
            plugin_path
        } else if info.fqcn.contains('\\') {
            fqcn_file_path(&info.fqcn)
        } else {
            let dir = Path::new(current_file)
                .parent()
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            if dir.is_empty() {
                format!("{}.ts", info.alias)
            } else {
                format!("{}/{}.ts", dir, info.alias)
            }
        };
        let relative = relative_import_path(current_file, &target);
        let entry = imports.entry(relative).or_default();
        entry.insert(info.alias.clone());
        if info.from_plugin_file {
            entry.insert(format!("{}Schema", info.alias));
        } else {
            entry.insert(format!("{}Schema", info.alias));
            if !info.is_enum {
                entry.insert(format!("{}Encoded", info.alias));
            }
        }
    }
}

fn transformer_file_path_for(fqcn: &str, transformers: &[Box<dyn Transformer>]) -> Option<String> {
    let temp = crate::ir::TypeIr::class_ref(fqcn);
    for transformer in transformers {
        if !transformer.provides_file() {
            continue;
        }
        if transformer.can_transform(
            crate::plugins::TransformInput::Type(&temp),
            WriterContext::Interface,
            &[],
            &[],
        ) {
            return transformer.file_path();
        }
    }
    None
}

fn assemble_file(imports: &BTreeMap<String, BTreeSet<String>>, exports: &[String]) -> String {
    let exports_joined = exports.join("\n\n");
    let has_effect = exports_joined.contains("S.") || exports_joined.contains("Schema");
    let mut content = String::new();
    if has_effect {
        content.push_str("import { Schema as S } from 'effect';\n\n");
    }
    if !imports.is_empty() {
        content.push_str(&write_imports(imports));
        content.push('\n');
    }
    content.push_str(&exports_joined);
    content
}

fn write_imports(imports: &BTreeMap<String, BTreeSet<String>>) -> String {
    let mut lines = Vec::new();
    for (relative_path, names) in imports {
        let import_path = relative_path.strip_suffix(".ts").unwrap_or(relative_path);
        let names_list: Vec<String> = names
            .iter()
            .map(|name| {
                if name.ends_with("Schema") {
                    name.clone()
                } else {
                    format!("type {name}")
                }
            })
            .collect();
        lines.push(format!(
            "import {{ {} }} from '{import_path}';",
            names_list.join(", ")
        ));
    }
    lines.join("\n")
}
