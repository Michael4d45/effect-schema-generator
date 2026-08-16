use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::{Identifier, LocalIdentifier};

pub(crate) fn owned_name(bytes: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(bytes).ok()?.trim();
    if s.is_empty() {
        return None;
    }
    let trimmed = s.trim_start_matches('\\');
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub(crate) fn bytes_to_string(bytes: &[u8]) -> String {
    owned_name(bytes).unwrap_or_else(|| String::from_utf8_lossy(bytes).into_owned())
}

pub(crate) fn short_name(fqcn: &str) -> &str {
    fqcn.rsplit('\\').next().unwrap_or(fqcn)
}

pub(crate) fn join_fqcn(prefix: &str, name: &str) -> String {
    let prefix = prefix.trim_start_matches('\\').trim_end_matches('\\');
    let name = name.trim_start_matches('\\');
    if prefix.is_empty() {
        name.to_string()
    } else if name.is_empty() {
        prefix.to_string()
    } else {
        format!("{prefix}\\{name}")
    }
}

pub(crate) fn resolve_local(
    resolved: &ResolvedNames<'_>,
    name: &LocalIdentifier<'_>,
    namespace: Option<&str>,
) -> Option<String> {
    if let Some(bytes) = resolved.resolve(&name.span.start) {
        return owned_name(bytes);
    }
    let local = owned_name(name.value)?;
    match namespace {
        Some(ns) if !ns.is_empty() => Some(format!("{ns}\\{local}")),
        _ => Some(local),
    }
}

pub(crate) fn resolve_identifier(
    resolved: &ResolvedNames<'_>,
    ident: &Identifier<'_>,
    namespace: Option<&str>,
) -> String {
    let bytes = resolved
        .resolve(&ident.span().start)
        .unwrap_or_else(|| ident.value());
    let name = owned_name(bytes).unwrap_or_else(|| bytes_to_string(ident.value()));
    if name.contains('\\') {
        name
    } else if let Some(ns) = namespace.filter(|ns| !ns.is_empty()) {
        format!("{ns}\\{name}")
    } else {
        name
    }
}
