mod native;
mod phpdoc;

use std::collections::HashSet;

use crate::ir::{EnumIr, NamespaceIr, PropertyIr, RootIr, SchemaIr, TypeIr};
use crate::tokens::{ClassToken, PublicPropertyToken, Token};

use self::native::{build_native_type, native_allows_null};
use self::phpdoc::build_phpdoc_type;

/// Build RootIR from parsed class/enum tokens.
pub fn build(tokens: &[Token]) -> RootIr {
    let mut root = RootIr::default();
    let mut enum_fqcns = HashSet::new();

    for token in tokens {
        let namespace = match token {
            Token::Class(class) => class.namespace.clone(),
            Token::Enum(enum_token) => {
                enum_fqcns.insert(enum_token.fqcn.trim_start_matches('\\').to_string());
                enum_token.namespace.clone()
            }
        };
        root.namespaces
            .entry(namespace.clone())
            .or_insert_with(|| NamespaceIr::new(namespace));
    }

    for token in tokens {
        if let Token::Enum(enum_token) = token {
            if let Some(ns) = root.namespaces.get_mut(&enum_token.namespace) {
                ns.enums.push(EnumIr {
                    name: enum_token.name.clone(),
                    cases: enum_token.cases.clone(),
                    backed_type: enum_token.backed_type.clone(),
                });
            }
        }
    }

    for token in tokens {
        if let Token::Class(class) = token {
            let schema = build_schema(class);
            if let Some(ns) = root.namespaces.get_mut(&class.namespace) {
                ns.schemas.push(schema);
            }
        }
    }

    for ns in root.namespaces.values_mut() {
        for schema in &mut ns.schemas {
            for property in &mut schema.properties {
                property.ty.mark_enums(&enum_fqcns);
            }
        }
    }

    root
}

fn build_schema(class: &ClassToken) -> SchemaIr {
    let properties = class
        .public_properties
        .iter()
        .map(|property| build_property(property, class))
        .collect();

    SchemaIr {
        name: class.name.clone(),
        uses: class.uses.clone(),
        properties,
        class_attributes: class.attributes.clone(),
    }
}

fn build_property(property: &PublicPropertyToken, class: &ClassToken) -> PropertyIr {
    let ty = build_type(property, class);
    let nullable = ty.is_nullable() || native_allows_null(&property.native_type);
    PropertyIr {
        name: property.name.clone(),
        ty,
        nullable,
        optional: false,
        attributes: property.attributes.clone(),
        hidden: false,
    }
}

fn build_type(property: &PublicPropertyToken, class: &ClassToken) -> TypeIr {
    if let Some(phpdoc) = property.phpdoc_type.as_deref() {
        if !phpdoc.trim().is_empty() {
            return build_phpdoc_type(phpdoc, &class.uses, &class.namespace);
        }
    }
    build_native_type(&property.native_type)
}
