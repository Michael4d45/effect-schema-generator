use std::borrow::Cow;
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::{Identifier, LocalIdentifier, Statement};
use mago_syntax::parser::parse_file;

/// A class or enum declaration extracted from a PHP file.
#[derive(Debug, Clone)]
pub(crate) struct Declaration {
    pub path: PathBuf,
    pub fqcn: String,
    pub kind: DeclarationKind,
}

#[derive(Debug, Clone)]
pub(crate) enum DeclarationKind {
    Class { extends: Option<String> },
    Enum,
}

/// Parse one PHP file and copy owned FQCNs out of the mago arena.
pub(crate) fn extract_declarations(path: &Path) -> Vec<Declaration> {
    let Ok(contents) = std::fs::read(path) else {
        return Vec::new();
    };
    if contents.is_empty() {
        return Vec::new();
    }

    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Owned(path.to_string_lossy().into_owned().into_bytes()),
        Cow::Owned(contents),
    );
    let program = parse_file(&arena, &file);
    if program.has_errors() {
        return Vec::new();
    }

    let resolver = NameResolver::new(&arena);
    let resolved = resolver.resolve(program);

    let mut out = Vec::new();
    collect_statements(path, &program.statements, None, &resolved, &mut out);
    out
}

fn collect_statements(
    path: &Path,
    statements: &mago_syntax::cst::Sequence<'_, Statement<'_>>,
    namespace: Option<&str>,
    resolved: &ResolvedNames<'_>,
    out: &mut Vec<Declaration>,
) {
    for statement in statements {
        match statement {
            Statement::Namespace(ns) => {
                let name = ns
                    .name
                    .as_ref()
                    .and_then(bytes_ident)
                    .map(|s| s.trim_start_matches('\\').to_string());
                collect_statements(path, ns.statements(), name.as_deref(), resolved, out);
            }
            Statement::Block(block) => {
                collect_statements(path, &block.statements, namespace, resolved, out);
            }
            Statement::Class(class) => {
                if let Some(fqcn) = resolve_local(resolved, &class.name, namespace) {
                    let extends = class
                        .extends
                        .as_ref()
                        .and_then(|ext| ext.types.nodes.first())
                        .and_then(|ident| resolve_identifier(resolved, ident, namespace));
                    out.push(Declaration {
                        path: path.to_path_buf(),
                        fqcn,
                        kind: DeclarationKind::Class { extends },
                    });
                }
            }
            Statement::Enum(enum_) => {
                if let Some(fqcn) = resolve_local(resolved, &enum_.name, namespace) {
                    out.push(Declaration {
                        path: path.to_path_buf(),
                        fqcn,
                        kind: DeclarationKind::Enum,
                    });
                }
            }
            _ => {}
        }
    }
}

fn resolve_local(
    resolved: &ResolvedNames<'_>,
    name: &LocalIdentifier<'_>,
    namespace: Option<&str>,
) -> Option<String> {
    if let Some(bytes) = resolved.resolve(&name.span.start) {
        return require_namespaced(owned_name(bytes)?);
    }
    let local = owned_name(name.value)?;
    match namespace {
        Some(ns) if !ns.is_empty() => Some(format!("{ns}\\{local}")),
        _ => None,
    }
}

fn resolve_identifier(
    resolved: &ResolvedNames<'_>,
    ident: &Identifier<'_>,
    namespace: Option<&str>,
) -> Option<String> {
    let bytes = resolved
        .resolve(&ident.span().start)
        .unwrap_or_else(|| ident.value());
    let name = owned_name(bytes)?;
    if name.contains('\\') {
        Some(name)
    } else if let Some(ns) = namespace.filter(|ns| !ns.is_empty()) {
        Some(format!("{ns}\\{name}"))
    } else {
        Some(name)
    }
}

fn bytes_ident(ident: &Identifier<'_>) -> Option<String> {
    owned_name(ident.value())
}

fn owned_name(bytes: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(bytes).ok()?.trim();
    if s.is_empty() {
        return None;
    }
    let trimmed = s.trim_start_matches('\\');
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

fn require_namespaced(name: String) -> Option<String> {
    if name.contains('\\') {
        Some(name)
    } else {
        None
    }
}
