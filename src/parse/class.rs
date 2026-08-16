use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::{
    Class, ClassLikeMember, FunctionLikeParameter, Method, Program, Property, Statement, Trait,
    TraitUse,
};
use mago_syntax::parser::parse_file;

use crate::error::{Error, Result};
use crate::ir::AttributeIr;
use crate::tokens::{ClassToken, NativeType, PublicPropertyToken};

use super::names::{owned_name, resolve_identifier, resolve_local, short_name};
use super::phpdoc::{doc_tags_between, phpdoc_for_property, DocTags};
use super::types::{convert_attributes, convert_hint, is_public_non_static, variable_name};
use super::uses::collect_use_map;

#[derive(Debug, Clone)]
struct ClassLikeInfo {
    namespace: String,
    fqcn: String,
    file: PathBuf,
    uses: BTreeMap<String, String>,
    properties: Vec<PublicPropertyToken>,
    attributes: Vec<AttributeIr>,
    parent: Option<String>,
    traits: Vec<String>,
}

struct Loader {
    cache: HashMap<String, Option<ClassLikeInfo>>,
}

impl Loader {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn load(&mut self, fqcn: &str, from_file: &Path, from_ns: &str) -> Option<ClassLikeInfo> {
        if let Some(cached) = self.cache.get(fqcn) {
            return cached.clone();
        }
        let info = find_class_file(from_file, from_ns, fqcn)
            .and_then(|path| parse_class_like_file(&path, fqcn).ok().flatten());
        self.cache.insert(fqcn.to_string(), info.clone());
        info
    }
}

pub fn parse_data_class(fqcn: &str, file: &Path) -> Result<ClassToken> {
    let info = parse_class_like_file(file, fqcn)?
        .ok_or_else(|| Error::Parse(format!("class {fqcn} not found in {}", file.display())))?;
    let mut loader = Loader::new();
    loader.cache.insert(fqcn.to_string(), Some(info.clone()));

    let uses = merge_uses(&info, &mut loader);
    let public_properties = flatten_properties(&info, &mut loader);
    let name = short_name(&info.fqcn).to_string();

    Ok(ClassToken {
        namespace: info.namespace,
        fqcn: info.fqcn,
        name,
        uses,
        public_properties,
        attributes: info.attributes,
    })
}

fn merge_uses(info: &ClassLikeInfo, loader: &mut Loader) -> BTreeMap<String, String> {
    let mut uses = info.uses.clone();
    let mut visited = HashSet::new();
    merge_related_uses(info, loader, &mut uses, &mut visited);
    uses
}

fn merge_related_uses(
    info: &ClassLikeInfo,
    loader: &mut Loader,
    uses: &mut BTreeMap<String, String>,
    visited: &mut HashSet<String>,
) {
    if !visited.insert(info.fqcn.clone()) {
        return;
    }
    for trait_fqcn in &info.traits {
        if let Some(trait_info) = loader.load(trait_fqcn, &info.file, &info.namespace) {
            for (alias, fqcn) in &trait_info.uses {
                uses.entry(alias.clone()).or_insert_with(|| fqcn.clone());
            }
            merge_related_uses(&trait_info, loader, uses, visited);
        }
    }
    if let Some(parent) = &info.parent {
        if let Some(parent_info) = loader.load(parent, &info.file, &info.namespace) {
            for (alias, fqcn) in &parent_info.uses {
                uses.entry(alias.clone()).or_insert_with(|| fqcn.clone());
            }
            merge_related_uses(&parent_info, loader, uses, visited);
        }
    }
}

fn flatten_properties(info: &ClassLikeInfo, loader: &mut Loader) -> Vec<PublicPropertyToken> {
    let mut properties = Vec::new();
    let mut seen = HashSet::new();
    let mut visited = HashSet::new();
    collect_properties(info, loader, &mut properties, &mut seen, &mut visited);
    properties
}

fn collect_properties(
    info: &ClassLikeInfo,
    loader: &mut Loader,
    properties: &mut Vec<PublicPropertyToken>,
    seen: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) {
    if !visited.insert(info.fqcn.clone()) {
        return;
    }

    for trait_fqcn in &info.traits {
        if let Some(trait_info) = loader.load(trait_fqcn, &info.file, &info.namespace) {
            collect_properties(&trait_info, loader, properties, seen, visited);
        }
    }

    for property in &info.properties {
        if seen.insert(property.name.clone()) {
            properties.push(property.clone());
        }
    }

    if let Some(parent) = &info.parent {
        if let Some(parent_info) = loader.load(parent, &info.file, &info.namespace) {
            collect_properties(&parent_info, loader, properties, seen, visited);
        }
    }
}

fn parse_class_like_file(path: &Path, fqcn: &str) -> Result<Option<ClassLikeInfo>> {
    let contents = std::fs::read(path)?;
    if contents.is_empty() {
        return Ok(None);
    }

    let arena = LocalArena::new();
    let file = File::ephemeral(
        std::borrow::Cow::Owned(path.to_string_lossy().into_owned().into_bytes()),
        std::borrow::Cow::Owned(contents),
    );
    let program = parse_file(&arena, &file);
    let resolver = NameResolver::new(&arena);
    let resolved = resolver.resolve(program);
    let uses = collect_use_map(&program.statements);

    Ok(find_class_like(
        path,
        fqcn,
        &arena,
        program,
        &resolved,
        uses,
        &program.statements,
        None,
    ))
}

#[allow(clippy::too_many_arguments)]
fn find_class_like(
    path: &Path,
    fqcn: &str,
    arena: &LocalArena,
    program: &Program<'_>,
    resolved: &ResolvedNames<'_>,
    uses: BTreeMap<String, String>,
    statements: &mago_syntax::cst::Sequence<'_, Statement<'_>>,
    namespace: Option<&str>,
) -> Option<ClassLikeInfo> {
    let want = short_name(fqcn);
    for statement in statements {
        match statement {
            Statement::Namespace(ns) => {
                let name = ns.name.as_ref().and_then(|ident| owned_name(ident.value()));
                if let Some(found) = find_class_like(
                    path,
                    fqcn,
                    arena,
                    program,
                    resolved,
                    uses.clone(),
                    ns.statements(),
                    name.as_deref().or(namespace),
                ) {
                    return Some(found);
                }
            }
            Statement::Block(block) => {
                if let Some(found) = find_class_like(
                    path,
                    fqcn,
                    arena,
                    program,
                    resolved,
                    uses.clone(),
                    &block.statements,
                    namespace,
                ) {
                    return Some(found);
                }
            }
            Statement::Class(class) => {
                let resolved_fqcn = resolve_local(resolved, &class.name, namespace)?;
                if resolved_fqcn == fqcn || owned_name(class.name.value).as_deref() == Some(want) {
                    return Some(class_info(
                        path,
                        arena,
                        program,
                        resolved,
                        uses,
                        class,
                        resolved_fqcn,
                        namespace.unwrap_or("").to_string(),
                    ));
                }
            }
            Statement::Trait(trait_) => {
                let resolved_fqcn = resolve_local(resolved, &trait_.name, namespace)?;
                if resolved_fqcn == fqcn || owned_name(trait_.name.value).as_deref() == Some(want) {
                    return Some(trait_info(
                        path,
                        arena,
                        program,
                        resolved,
                        uses,
                        trait_,
                        resolved_fqcn,
                        namespace.unwrap_or("").to_string(),
                    ));
                }
            }
            _ => {}
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn class_info(
    path: &Path,
    arena: &LocalArena,
    program: &Program<'_>,
    resolved: &ResolvedNames<'_>,
    uses: BTreeMap<String, String>,
    class: &Class<'_>,
    fqcn: String,
    namespace: String,
) -> ClassLikeInfo {
    let ns = if namespace.is_empty() {
        None
    } else {
        Some(namespace.as_str())
    };
    let attributes = convert_attributes(&class.attribute_lists, resolved, ns);
    let parent = class
        .extends
        .as_ref()
        .and_then(|ext| ext.types.nodes.first())
        .map(|ident| resolve_identifier(resolved, ident, ns));
    let traits = collect_trait_names(&class.members, resolved, ns);
    let properties = extract_members_properties(
        arena,
        program,
        resolved,
        ns,
        &fqcn,
        &class.members,
        class.left_brace.end.offset,
        class.span().start.offset,
    );

    ClassLikeInfo {
        namespace,
        fqcn,
        file: path.to_path_buf(),
        uses,
        properties,
        attributes,
        parent,
        traits,
    }
}

#[allow(clippy::too_many_arguments)]
fn trait_info(
    path: &Path,
    arena: &LocalArena,
    program: &Program<'_>,
    resolved: &ResolvedNames<'_>,
    uses: BTreeMap<String, String>,
    trait_: &Trait<'_>,
    fqcn: String,
    namespace: String,
) -> ClassLikeInfo {
    let ns = if namespace.is_empty() {
        None
    } else {
        Some(namespace.as_str())
    };
    let attributes = convert_attributes(&trait_.attribute_lists, resolved, ns);
    let traits = collect_trait_names(&trait_.members, resolved, ns);
    let properties = extract_members_properties(
        arena,
        program,
        resolved,
        ns,
        &fqcn,
        &trait_.members,
        trait_.left_brace.end.offset,
        trait_.span().start.offset,
    );

    ClassLikeInfo {
        namespace,
        fqcn,
        file: path.to_path_buf(),
        uses,
        properties,
        attributes,
        parent: None,
        traits,
    }
}

fn collect_trait_names(
    members: &mago_syntax::cst::Sequence<'_, ClassLikeMember<'_>>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
) -> Vec<String> {
    let mut names = Vec::new();
    for member in members {
        if let ClassLikeMember::TraitUse(trait_use) = member {
            names.extend(trait_use_names(trait_use, resolved, namespace));
        }
    }
    names
}

fn trait_use_names(
    trait_use: &TraitUse<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
) -> Vec<String> {
    trait_use
        .trait_names
        .iter()
        .map(|ident| resolve_identifier(resolved, ident, namespace))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn extract_members_properties(
    arena: &LocalArena,
    program: &Program<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
    current_fqcn: &str,
    members: &mago_syntax::cst::Sequence<'_, ClassLikeMember<'_>>,
    body_start: u32,
    class_span_start: u32,
) -> Vec<PublicPropertyToken> {
    let class_doc = doc_tags_between(arena, program, 0, class_span_start);
    let constructor = members.iter().find_map(|member| match member {
        ClassLikeMember::Method(method) if is_constructor(method) => Some(method),
        _ => None,
    });
    let constructor_doc = constructor.and_then(|method| {
        let prev = previous_member_end(members, method.span().start.offset, body_start);
        doc_tags_between(arena, program, prev, method.span().start.offset)
    });

    let mut properties = Vec::new();
    let mut prev_end = body_start;

    for member in members {
        match member {
            ClassLikeMember::Property(property) => {
                let core = property_core_start(property);
                let own_doc = doc_tags_between(arena, program, prev_end, core);
                properties.extend(plain_properties(
                    property,
                    resolved,
                    namespace,
                    current_fqcn,
                    own_doc.as_ref(),
                    constructor_doc.as_ref(),
                    class_doc.as_ref(),
                ));
                prev_end = property.span().end.offset;
            }
            ClassLikeMember::Method(method) if is_constructor(method) => {
                properties.extend(promoted_properties(
                    arena,
                    program,
                    method,
                    resolved,
                    namespace,
                    current_fqcn,
                    constructor_doc.as_ref(),
                    class_doc.as_ref(),
                ));
                prev_end = method.span().end.offset;
            }
            other => {
                prev_end = other.span().end.offset;
            }
        }
    }

    properties
}

fn previous_member_end(
    members: &mago_syntax::cst::Sequence<'_, ClassLikeMember<'_>>,
    before: u32,
    body_start: u32,
) -> u32 {
    members
        .iter()
        .filter(|member| member.span().end.offset <= before)
        .map(|member| member.span().end.offset)
        .max()
        .unwrap_or(body_start)
}

fn is_constructor(method: &Method<'_>) -> bool {
    method.name.value.eq_ignore_ascii_case(b"__construct")
}

fn property_core_start(property: &Property<'_>) -> u32 {
    if let Some(modifier) = property.modifiers().first() {
        return modifier.span().start.offset;
    }
    if let Some(hint) = property.hint() {
        return hint.span().start.offset;
    }
    property.first_variable().span.start.offset
}

fn parameter_core_start(param: &FunctionLikeParameter<'_>) -> u32 {
    if let Some(modifier) = param.modifiers.first() {
        return modifier.span().start.offset;
    }
    if let Some(hint) = &param.hint {
        return hint.span().start.offset;
    }
    param.variable.span.start.offset
}

fn plain_properties(
    property: &Property<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
    current_fqcn: &str,
    own_doc: Option<&DocTags>,
    constructor_doc: Option<&DocTags>,
    class_doc: Option<&DocTags>,
) -> Vec<PublicPropertyToken> {
    if !is_public_non_static(property.modifiers()) {
        return Vec::new();
    }
    let native_type = property
        .hint()
        .map(|hint| convert_hint(hint, resolved, namespace, current_fqcn))
        .unwrap_or(NativeType::String);
    let attributes = match property {
        Property::Plain(plain) => convert_attributes(&plain.attribute_lists, resolved, namespace),
        Property::Hooked(hooked) => {
            convert_attributes(&hooked.attribute_lists, resolved, namespace)
        }
    };

    property
        .variables()
        .into_iter()
        .map(|variable| {
            let name = variable_name(variable.name);
            let phpdoc_type = phpdoc_for_property(own_doc, constructor_doc, class_doc, &name);
            PublicPropertyToken {
                name,
                native_type: native_type.clone(),
                phpdoc_type,
                attributes: attributes.clone(),
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn promoted_properties(
    arena: &LocalArena,
    program: &Program<'_>,
    method: &Method<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
    current_fqcn: &str,
    constructor_doc: Option<&DocTags>,
    class_doc: Option<&DocTags>,
) -> Vec<PublicPropertyToken> {
    let mut properties = Vec::new();
    let mut prev_end = method.parameter_list.left_parenthesis.end.offset;
    for param in method.parameter_list.parameters.iter() {
        let core = parameter_core_start(param);
        let own_doc = doc_tags_between(arena, program, prev_end, core);
        prev_end = param.span().end.offset;

        if !param.is_promoted_property() || !is_public_non_static(&param.modifiers) {
            continue;
        }
        let name = variable_name(param.variable.name);
        let native_type = param
            .hint
            .as_ref()
            .map(|hint| convert_hint(hint, resolved, namespace, current_fqcn))
            .unwrap_or(NativeType::String);
        let phpdoc_type = phpdoc_for_property(own_doc.as_ref(), constructor_doc, class_doc, &name);
        properties.push(PublicPropertyToken {
            name,
            native_type,
            phpdoc_type,
            attributes: convert_attributes(&param.attribute_lists, resolved, namespace),
        });
    }
    properties
}

fn find_class_file(from_file: &Path, from_ns: &str, fqcn: &str) -> Option<PathBuf> {
    let from_dir = from_file.parent()?;
    let from_ns = from_ns.trim_matches('\\');
    let fqcn = fqcn.trim_matches('\\');

    if let Some(rest) = fqcn.strip_prefix(from_ns) {
        let rest = rest.trim_start_matches('\\');
        if !rest.is_empty() {
            let mut path = from_dir.to_path_buf();
            let parts: Vec<&str> = rest.split('\\').collect();
            for (i, part) in parts.iter().enumerate() {
                if i + 1 == parts.len() {
                    path.push(format!("{part}.php"));
                } else {
                    path.push(part);
                }
            }
            if path.is_file() {
                return Some(path);
            }
        }
    }

    let short = short_name(fqcn);
    let same_dir = from_dir.join(format!("{short}.php"));
    if same_dir.is_file() {
        Some(same_dir)
    } else {
        None
    }
}
