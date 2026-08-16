use std::path::Path;

use mago_allocator::LocalArena;
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_names::ResolvedNames;
use mago_syntax::cst::{ClassLikeMember, EnumCaseItem, Hint, Statement};
use mago_syntax::parser::parse_file;

use crate::error::{Error, Result};
use crate::ir::EnumCaseIr;
use crate::tokens::EnumToken;

use super::names::{owned_name, resolve_local, short_name};
use super::types::expr_to_json;

pub fn parse_enum(fqcn: &str, file: &Path) -> Result<EnumToken> {
    let contents = std::fs::read(file)?;
    if contents.is_empty() {
        return Err(Error::Parse(format!("empty enum file {}", file.display())));
    }

    let arena = LocalArena::new();
    let mago_file = File::ephemeral(
        std::borrow::Cow::Owned(file.to_string_lossy().into_owned().into_bytes()),
        std::borrow::Cow::Owned(contents),
    );
    let program = parse_file(&arena, &mago_file);
    let resolver = NameResolver::new(&arena);
    let resolved = resolver.resolve(program);

    find_enum(fqcn, &resolved, &program.statements, None)
        .ok_or_else(|| Error::Parse(format!("enum {fqcn} not found in {}", file.display())))
}

fn find_enum(
    fqcn: &str,
    resolved: &ResolvedNames<'_>,
    statements: &mago_syntax::cst::Sequence<'_, Statement<'_>>,
    namespace: Option<&str>,
) -> Option<EnumToken> {
    let want = short_name(fqcn);
    for statement in statements {
        match statement {
            Statement::Namespace(ns) => {
                let name = ns.name.as_ref().and_then(|ident| owned_name(ident.value()));
                if let Some(found) = find_enum(
                    fqcn,
                    resolved,
                    ns.statements(),
                    name.as_deref().or(namespace),
                ) {
                    return Some(found);
                }
            }
            Statement::Block(block) => {
                if let Some(found) = find_enum(fqcn, resolved, &block.statements, namespace) {
                    return Some(found);
                }
            }
            Statement::Enum(enum_) => {
                let resolved_fqcn = resolve_local(resolved, &enum_.name, namespace)?;
                if resolved_fqcn == fqcn || owned_name(enum_.name.value).as_deref() == Some(want) {
                    let namespace = namespace.unwrap_or("").to_string();
                    let name = short_name(&resolved_fqcn).to_string();
                    let backed_type = enum_
                        .backing_type_hint
                        .as_ref()
                        .map(|hint| backing_type(&hint.hint))
                        .unwrap_or_default();
                    let cases = enum_
                        .members
                        .iter()
                        .filter_map(|member| match member {
                            ClassLikeMember::EnumCase(case) => Some(convert_case(&case.item)),
                            _ => None,
                        })
                        .collect();
                    return Some(EnumToken {
                        fqcn: resolved_fqcn,
                        namespace,
                        name,
                        backed_type,
                        cases,
                    });
                }
            }
            _ => {}
        }
    }
    None
}

fn backing_type(hint: &Hint<'_>) -> String {
    match hint {
        Hint::String(_) => "string".to_string(),
        Hint::Integer(_) => "int".to_string(),
        Hint::Identifier(ident) => owned_name(ident.value()).unwrap_or_default(),
        _ => String::new(),
    }
}

fn convert_case(item: &EnumCaseItem<'_>) -> EnumCaseIr {
    let name = owned_name(item.name().value).unwrap_or_default();
    let value = match item {
        EnumCaseItem::Unit(_) => None,
        EnumCaseItem::Backed(backed) => {
            Some(expr_to_json(backed.value, &ResolvedNames::default(), None))
        }
    };
    EnumCaseIr { name, value }
}
