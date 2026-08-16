use crate::ir::{PropertyIr, TypeIr, WriterContext};
use crate::plugins::{TransformInput, Transformer};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ReferencedType {
    pub alias: String,
    pub fqcn: String,
    pub is_enum: bool,
    pub from_plugin_file: bool,
}

pub fn transform_type(
    ty: &TypeIr,
    context: WriterContext,
    transformers: &[Box<dyn Transformer>],
) -> String {
    for transformer in transformers {
        if transformer.can_transform(TransformInput::Type(ty), context, &[], &[]) {
            return transformer.transform(
                TransformInput::Type(ty),
                context,
                &[],
                &[],
                &|inner, ctx| transform_type(inner, ctx, transformers),
            );
        }
    }
    default_transform_with(ty, context, &|inner, ctx| {
        transform_type(inner, ctx, transformers)
    })
}

pub fn default_transform(ty: &TypeIr, context: WriterContext) -> String {
    default_transform_with(ty, context, &|inner, ctx| default_transform(inner, ctx))
}

fn default_transform_with(
    ty: &TypeIr,
    context: WriterContext,
    recurse: &dyn Fn(&TypeIr, WriterContext) -> String,
) -> String {
    match ty {
        TypeIr::Class { alias, is_enum, .. } => {
            if context == WriterContext::Schema {
                if *is_enum {
                    format!("{alias}Schema")
                } else {
                    format!("S.suspend(() => {alias}Schema)")
                }
            } else if context == WriterContext::EncodedInterface && !is_enum {
                format!("{alias}Encoded")
            } else {
                alias.clone()
            }
        }
        TypeIr::String => match context {
            WriterContext::Schema => "S.String".into(),
            _ => "string".into(),
        },
        TypeIr::Int | TypeIr::Float => match context {
            WriterContext::Schema => "S.Number".into(),
            _ => "number".into(),
        },
        TypeIr::Bool => match context {
            WriterContext::Schema => "S.Boolean".into(),
            _ => "boolean".into(),
        },
        TypeIr::Array { item } => match item {
            Some(item) => {
                let item_type = recurse(item, context);
                if context == WriterContext::Schema {
                    format!("S.Array({item_type})")
                } else {
                    format!("readonly {item_type}[]")
                }
            }
            None => {
                if context == WriterContext::Schema {
                    "S.Array(S.Unknown)".into()
                } else {
                    "readonly unknown[]".into()
                }
            }
        },
        TypeIr::Nullable { inner } => {
            let inner_type = recurse(inner, context);
            if context == WriterContext::Schema {
                format!("S.NullOr({inner_type})")
            } else {
                format!("{inner_type} | null")
            }
        }
        TypeIr::Record { key, value } => {
            let key_type = recurse(key, context);
            let value_type = recurse(value, context);
            if context == WriterContext::Schema {
                format!("S.Record({{ key: {key_type}, value: {value_type} }})")
            } else {
                format!("Record<{key_type}, {value_type}>")
            }
        }
        TypeIr::Struct { properties } => {
            let mut props = Vec::new();
            for property in properties.iter().filter(|p| !p.hidden) {
                let prop_type = recurse(&property.ty, context);
                let optional = if property.optional { "?" } else { "" };
                if context == WriterContext::Schema {
                    props.push(format!("  {}: {prop_type}", property.name));
                } else {
                    props.push(format!(
                        "  readonly {}{optional}: {prop_type};",
                        property.name
                    ));
                }
            }
            if context == WriterContext::Schema {
                format!("S.Struct({{\n{}\n}})", props.join(",\n"))
            } else {
                format!("{{\n{}\n}}", props.join("\n"))
            }
        }
        TypeIr::Union { types } => transform_union(types, context, recurse),
        TypeIr::Unknown => {
            if context == WriterContext::Schema {
                "S.Unknown".into()
            } else {
                "unknown".into()
            }
        }
    }
}

fn transform_union(
    types: &[TypeIr],
    context: WriterContext,
    recurse: &dyn Fn(&TypeIr, WriterContext) -> String,
) -> String {
    let mut union_types = Vec::new();
    let mut has_null = false;
    for member in types {
        let effective = if let TypeIr::Nullable { inner } = member {
            has_null = true;
            inner.as_ref()
        } else {
            member
        };
        let transformed = recurse(effective, context);
        if transformed.contains(" | null")
            || transformed.contains("null |")
            || transformed.contains("S.Null")
            || transformed.contains("Null")
        {
            has_null = true;
        }
        if !union_types.contains(&transformed) {
            union_types.push(transformed);
        }
    }

    if context == WriterContext::Schema {
        let union_str = if union_types.len() > 1 {
            format!("S.Union({})", union_types.join(", "))
        } else {
            union_types
                .first()
                .cloned()
                .unwrap_or_else(|| "S.Unknown".into())
        };
        if has_null && !union_str.contains("S.Null") && !union_str.contains("NullOr") {
            format!("S.NullOr({union_str})")
        } else {
            union_str
        }
    } else {
        let union_str = union_types.join(" | ");
        if has_null && !union_str.contains("null") {
            format!("{union_str} | null")
        } else {
            union_str
        }
    }
}

pub fn write_property_line(prop: &PropertyIr, ty_str: &str, context: WriterContext) -> String {
    if context == WriterContext::Schema {
        let mut schema_ty = ty_str.to_string();
        if prop.nullable && !schema_ty.contains("Null") {
            schema_ty = format!("S.NullOr({schema_ty})");
        }
        if prop.optional {
            schema_ty = format!("S.optional({schema_ty})");
        }
        if schema_ty.contains('\n') {
            schema_ty = schema_ty.replace('\n', "\n  ");
        }
        format!("  {}: {schema_ty}", prop.name)
    } else {
        let mut ts = ty_str.to_string();
        if prop.nullable && !ts.contains("null") {
            ts.push_str(" | null");
        }
        let optional = if prop.optional { "?" } else { "" };
        let optional_pipe = if prop.optional { " | undefined" } else { "" };
        if ts.contains('\n') {
            ts = ts.replace('\n', "\n  ");
        }
        format!("  readonly {}{optional}: {ts}{optional_pipe};", prop.name)
    }
}

pub fn collect_referenced_types(
    ty: &TypeIr,
    transformers: &[Box<dyn Transformer>],
    referenced: &mut BTreeMap<String, ReferencedType>,
) {
    match ty {
        TypeIr::Class {
            fqcn,
            alias,
            is_enum,
            type_parameters,
            ..
        } => {
            let mut handled = false;
            let mut from_plugin_file = false;
            for transformer in transformers {
                if transformer.can_transform(
                    TransformInput::Type(ty),
                    WriterContext::Interface,
                    &[],
                    &[],
                ) {
                    handled = true;
                    if transformer.provides_file() {
                        from_plugin_file = true;
                        referenced.insert(
                            fqcn.clone(),
                            ReferencedType {
                                alias: alias.clone(),
                                fqcn: fqcn.clone(),
                                is_enum: *is_enum,
                                from_plugin_file: true,
                            },
                        );
                    }
                    break;
                }
            }
            if !handled
                && (fqcn.starts_with("App\\")
                    || fqcn.starts_with("EffectSchemaGenerator\\")
                    || !fqcn.contains('\\'))
            {
                referenced.insert(
                    fqcn.clone(),
                    ReferencedType {
                        alias: alias.clone(),
                        fqcn: fqcn.clone(),
                        is_enum: *is_enum,
                        from_plugin_file,
                    },
                );
            }
            for param in type_parameters {
                collect_referenced_types(param, transformers, referenced);
            }
        }
        TypeIr::Nullable { inner } => {
            collect_referenced_types(inner, transformers, referenced);
        }
        TypeIr::Union { types } => {
            for member in types {
                collect_referenced_types(member, transformers, referenced);
            }
        }
        TypeIr::Array { item: Some(item) } => {
            collect_referenced_types(item, transformers, referenced);
        }
        TypeIr::Record { key, value } => {
            collect_referenced_types(key, transformers, referenced);
            collect_referenced_types(value, transformers, referenced);
        }
        TypeIr::Struct { properties } => {
            for property in properties.iter().filter(|p| !p.hidden) {
                collect_referenced_types(&property.ty, transformers, referenced);
            }
        }
        _ => {}
    }
}
