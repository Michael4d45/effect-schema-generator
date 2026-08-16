use mago_allocator::Arena;
use mago_allocator::LocalArena;
use mago_database::file::FileId;
use mago_docblock::document::{Document, TagKind};
use mago_docblock::tag::{parse_param_tag, parse_var_tag};
use mago_span::{Position, Span};
use mago_syntax::cst::{Program, Trivia, TriviaKind};

#[derive(Debug, Clone, Default)]
pub(crate) struct DocTags {
    pub var_type: Option<String>,
    pub params: std::collections::BTreeMap<String, String>,
}

impl DocTags {
    pub(crate) fn type_for(&self, param_name: Option<&str>) -> Option<String> {
        if let Some(name) = param_name {
            if let Some(ty) = self.params.get(name) {
                return Some(ty.clone());
            }
        }
        self.var_type
            .clone()
            .or_else(|| self.params.values().next().cloned())
    }

    pub(crate) fn phpdoc_for_property(&self, name: &str) -> Option<String> {
        self.var_type
            .clone()
            .or_else(|| self.params.get(name).cloned())
    }
}

/// Extract a `@var` type, or a named `@param` type, from a PHPDoc comment.
pub fn phpdoc_type_string(comment: &str, param_name: Option<&str>) -> Option<String> {
    let arena = LocalArena::new();
    let bytes = arena.alloc_slice_copy(comment.as_bytes());
    let span = Span::new(
        FileId::zero(),
        Position::new(0),
        Position::new(bytes.len() as u32),
    );
    #[allow(deprecated)]
    let document = mago_docblock::parse_phpdoc_with_span(&arena, bytes, span).ok()?;
    doc_tags_from_document(&document).type_for(param_name)
}

pub(crate) fn doc_tags_from_document(document: &Document<'_>) -> DocTags {
    let mut tags = DocTags::default();
    for tag in document.get_tags() {
        match tag.kind {
            TagKind::Var => {
                if tags.var_type.is_none() {
                    if let Ok(parsed) = parse_var_tag(tag.description, tag.description_span) {
                        tags.var_type = utf8_owned(&parsed.type_string.value);
                    }
                }
            }
            TagKind::Param => {
                if let Ok(parsed) = parse_param_tag(tag.description, tag.description_span) {
                    let name = parsed
                        .variable
                        .name
                        .strip_prefix(b"$")
                        .unwrap_or(&parsed.variable.name);
                    if let Some(name) = utf8_owned(name) {
                        if let Some(ty) = parsed.type_string.and_then(|t| utf8_owned(&t.value)) {
                            tags.params.entry(name).or_insert(ty);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    tags
}

pub(crate) fn parse_trivia_doc(arena: &LocalArena, trivia: &Trivia<'_>) -> Option<DocTags> {
    #[allow(deprecated)]
    let document = mago_docblock::parse_trivia(arena, trivia).ok()?;
    Some(doc_tags_from_document(&document))
}

/// Last docblock whose span sits in `[from, to]` (byte offsets).
pub(crate) fn docblock_between<'a>(
    program: &'a Program<'a>,
    from: u32,
    to: u32,
) -> Option<&'a Trivia<'a>> {
    program
        .trivia
        .iter()
        .filter(|trivia| trivia.kind == TriviaKind::DocBlockComment)
        .filter(|trivia| trivia.span.start.offset >= from && trivia.span.end.offset <= to)
        .max_by_key(|trivia| trivia.span.end.offset)
}

pub(crate) fn doc_tags_between(
    arena: &LocalArena,
    program: &Program<'_>,
    from: u32,
    to: u32,
) -> Option<DocTags> {
    parse_trivia_doc(arena, docblock_between(program, from, to)?)
}

pub(crate) fn phpdoc_for_property(
    own: Option<&DocTags>,
    constructor: Option<&DocTags>,
    class: Option<&DocTags>,
    name: &str,
) -> Option<String> {
    if let Some(own) = own {
        if let Some(ty) = own.phpdoc_for_property(name) {
            return Some(ty);
        }
    }
    if let Some(constructor) = constructor {
        if let Some(ty) = constructor.params.get(name) {
            return Some(ty.clone());
        }
    }
    if let Some(class) = class {
        class.params.get(name).cloned()
    } else {
        None
    }
}

fn utf8_owned(bytes: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(bytes).ok()?.trim();
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}
