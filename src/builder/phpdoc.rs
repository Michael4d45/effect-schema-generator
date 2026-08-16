use std::collections::BTreeMap;

use crate::ir::{PropertyIr, TypeIr};

/// Parse a PHPDoc type string into TypeIR.
pub fn build_phpdoc_type(
    phpdoc: &str,
    uses: &BTreeMap<String, String>,
    current_namespace: &str,
) -> TypeIr {
    let mut parser = Parser {
        input: phpdoc.trim(),
        pos: 0,
        uses,
        current_namespace,
    };
    let ty = parser.parse_union();
    parser.skip_ws();
    ty
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    uses: &'a BTreeMap<String, String>,
    current_namespace: &'a str,
}

impl<'a> Parser<'a> {
    fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let mut chars = self.remaining().chars();
        let ch = chars.next()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.bump();
        }
    }

    fn starts_with(&self, s: &str) -> bool {
        self.remaining().starts_with(s)
    }

    fn consume(&mut self, expected: char) -> bool {
        self.skip_ws();
        if self.peek() == Some(expected) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn parse_union(&mut self) -> TypeIr {
        self.skip_ws();
        let mut types = Vec::new();
        let mut has_null = false;
        loop {
            self.skip_ws();
            if self.is_null_ident() {
                has_null = true;
                self.consume_ident();
            } else {
                types.push(self.parse_nullable());
            }
            self.skip_ws();
            if !self.consume('|') {
                break;
            }
        }

        let inner = match types.len() {
            0 => TypeIr::Unknown,
            1 => types.remove(0),
            _ => TypeIr::Union { types },
        };
        if has_null {
            wrap_nullable(inner)
        } else {
            inner
        }
    }

    fn is_null_ident(&self) -> bool {
        let ident = peek_ident(self.remaining());
        ident == Some("null")
    }

    fn parse_nullable(&mut self) -> TypeIr {
        self.skip_ws();
        if self.consume('?') {
            wrap_nullable(self.parse_atomic())
        } else {
            self.parse_atomic()
        }
    }

    fn parse_atomic(&mut self) -> TypeIr {
        self.skip_ws();
        if self.consume('(') {
            let inner = self.parse_union();
            self.consume(')');
            return self.apply_array_suffixes(inner);
        }

        let ident = match self.consume_ident() {
            Some(ident) => ident,
            None => return TypeIr::Unknown,
        };

        self.skip_ws();
        if ident == "array" && self.peek() == Some('{') {
            let shape = self.parse_array_shape();
            return self.apply_array_suffixes(shape);
        }

        if self.peek() == Some('<') {
            let generic = self.parse_generic(ident);
            return self.apply_array_suffixes(generic);
        }

        let ty = self.build_identifier(&ident);
        self.apply_array_suffixes(ty)
    }

    fn parse_generic(&mut self, name: String) -> TypeIr {
        self.consume('<');
        let mut args = Vec::new();
        let mut arg_is_shape = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == Some('>') {
                break;
            }
            self.skip_ws();
            let is_shape = name == "array" && self.starts_with("array{");
            args.push(self.parse_union());
            arg_is_shape.push(is_shape);
            self.skip_ws();
            if !self.consume(',') {
                break;
            }
        }
        self.consume('>');

        match name.as_str() {
            "array" if args.len() == 2 => {
                let value = if arg_is_shape.get(1).copied().unwrap_or(false) {
                    TypeIr::Array {
                        item: Some(Box::new(args.remove(1))),
                    }
                } else {
                    args.remove(1)
                };
                let key = args.remove(0);
                TypeIr::Record {
                    key: Box::new(key),
                    value: Box::new(value),
                }
            }
            "array" | "list" if args.len() == 1 => TypeIr::Array {
                item: Some(Box::new(args.remove(0))),
            },
            _ => {
                let fqcn = resolve_class_name(&name, self.uses, self.current_namespace);
                TypeIr::class_ref_params(fqcn, args)
            }
        }
    }

    fn parse_array_shape(&mut self) -> TypeIr {
        self.consume('{');
        let mut properties = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == Some('}') {
                break;
            }
            let key = self.consume_shape_key().unwrap_or_default();
            self.skip_ws();
            let optional = self.consume('?');
            self.consume(':');
            let ty = self.parse_union();
            properties.push(PropertyIr {
                name: key,
                ty,
                nullable: false,
                optional,
                attributes: Vec::new(),
                hidden: false,
            });
            self.skip_ws();
            if !self.consume(',') {
                break;
            }
        }
        self.consume('}');
        TypeIr::Struct { properties }
    }

    fn apply_array_suffixes(&mut self, mut ty: TypeIr) -> TypeIr {
        loop {
            self.skip_ws();
            if self.starts_with("[]") {
                self.bump();
                self.bump();
                ty = TypeIr::Array {
                    item: Some(Box::new(ty)),
                };
            } else {
                break;
            }
        }
        ty
    }

    fn consume_ident(&mut self) -> Option<String> {
        self.skip_ws();
        let ident = peek_ident(self.remaining())?;
        self.pos += ident.len();
        Some(ident.to_string())
    }

    fn consume_shape_key(&mut self) -> Option<String> {
        self.skip_ws();
        if let Some(ident) = peek_ident(self.remaining()) {
            self.pos += ident.len();
            return Some(ident.to_string());
        }
        let rest = self.remaining();
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            None
        } else {
            self.pos += digits.len();
            Some(digits)
        }
    }

    fn build_identifier(&self, name: &str) -> TypeIr {
        match name.to_ascii_lowercase().as_str() {
            "string" => TypeIr::String,
            "int" | "integer" => TypeIr::Int,
            "float" | "double" => TypeIr::Float,
            "bool" | "boolean" => TypeIr::Bool,
            "array" => TypeIr::Array { item: None },
            "mixed" => TypeIr::Unknown,
            "array-key" => TypeIr::Union {
                types: vec![TypeIr::String, TypeIr::Int],
            },
            _ => {
                let fqcn = resolve_class_name(name, self.uses, self.current_namespace);
                TypeIr::class_ref(fqcn)
            }
        }
    }
}

fn peek_ident(input: &str) -> Option<&str> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let mut i = 0;
    if bytes[0] == b'\\' {
        i = 1;
    }
    if i >= bytes.len() {
        return None;
    }
    if !is_ident_start(bytes[i]) {
        return None;
    }
    i += 1;
    while i < bytes.len() {
        let b = bytes[i];
        if is_ident_continue(b) {
            i += 1;
        } else if b == b'\\' {
            if i + 1 < bytes.len() && is_ident_start(bytes[i + 1]) {
                i += 2;
                while i < bytes.len() && is_ident_continue(bytes[i]) {
                    i += 1;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    if i == 0 || (i == 1 && bytes[0] == b'\\') {
        None
    } else {
        Some(&input[..i])
    }
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

fn resolve_class_name(
    name: &str,
    uses: &BTreeMap<String, String>,
    current_namespace: &str,
) -> String {
    let trimmed = name.trim_start_matches('\\');
    if trimmed.contains('\\') {
        return trimmed.to_string();
    }

    if let Some(fqcn) = uses.get(trimmed) {
        return fqcn.trim_start_matches('\\').to_string();
    }

    for (alias, fqcn) in uses {
        let class_name = alias.rsplit('\\').next().unwrap_or(alias);
        if class_name == trimmed {
            return fqcn.trim_start_matches('\\').to_string();
        }
    }

    if trimmed == "Collection" {
        return r"Illuminate\Support\Collection".to_string();
    }

    if !current_namespace.is_empty() {
        return format!("{current_namespace}\\{trimmed}");
    }

    trimmed.to_string()
}

fn wrap_nullable(ty: TypeIr) -> TypeIr {
    if ty.is_nullable() {
        ty
    } else {
        TypeIr::Nullable {
            inner: Box::new(ty),
        }
    }
}
