mod collection;
mod const_object_enum;
mod date;
mod effect_schema_enum;
mod hidden;
mod lazy_optional;
mod length_aware_paginator;
mod snake_case;
mod snake_case_attribute;
mod type_enum;
mod type_map;
mod uploaded_file;

pub use collection::CollectionPlugin;
pub use const_object_enum::ConstObjectEnumWriter;
pub use date::DatePlugin;
pub use effect_schema_enum::EffectSchemaEnumWriter;
pub use hidden::HiddenPlugin;
pub use lazy_optional::LazyOptionalPlugin;
pub use length_aware_paginator::LengthAwarePaginatorPlugin;
pub use snake_case::SnakeCasePlugin;
pub use snake_case_attribute::SnakeCaseAttributePlugin;
pub use type_enum::TypeEnumWriter;
pub use type_map::TypeMapPlugin;
pub use uploaded_file::UploadedFilePlugin;

use crate::config::Config;
use crate::ir::{AttributeIr, EnumIr, PropertyIr, SchemaIr, TypeIr, WriterContext};

pub trait Transformer: Send + Sync {
    fn can_transform(
        &self,
        input: TransformInput<'_>,
        context: WriterContext,
        class_attributes: &[crate::ir::AttributeIr],
        property_attributes: &[crate::ir::AttributeIr],
    ) -> bool;

    fn transform(
        &self,
        input: TransformInput<'_>,
        context: WriterContext,
        class_attributes: &[crate::ir::AttributeIr],
        property_attributes: &[crate::ir::AttributeIr],
        inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String;

    fn provides_file(&self) -> bool {
        false
    }

    fn file_path(&self) -> Option<String> {
        None
    }

    fn file_content(&self) -> Option<String> {
        None
    }

    /// Mutate IR before writing (optional/hidden/snake_case).
    fn preprocess(&self, _root: &mut crate::ir::RootIr) {}
}

#[derive(Clone, Copy)]
pub enum TransformInput<'a> {
    Type(&'a TypeIr),
    Schema(&'a SchemaIr),
    Enum(&'a EnumIr),
    Property(&'a PropertyIr),
}

pub fn load_plugins(config: &Config) -> Vec<Box<dyn Transformer>> {
    let mut plugins: Vec<Box<dyn Transformer>> = config
        .plugins
        .iter()
        .filter_map(|name| -> Option<Box<dyn Transformer>> {
            match name.as_str() {
                "length_aware_paginator" => Some(Box::new(LengthAwarePaginatorPlugin)),
                "lazy_optional" => Some(Box::new(LazyOptionalPlugin)),
                "hidden" => Some(Box::new(HiddenPlugin)),
                "snake_case" => Some(Box::new(SnakeCasePlugin)),
                "snake_case_attribute" => Some(Box::new(SnakeCaseAttributePlugin)),
                "date" => Some(Box::new(DatePlugin)),
                "collection" => Some(Box::new(CollectionPlugin)),
                "type_enum" => Some(Box::new(TypeEnumWriter)),
                "effect_schema_enum" => Some(Box::new(EffectSchemaEnumWriter)),
                "const_object_enum" => Some(Box::new(ConstObjectEnumWriter)),
                "uploaded_file" => Some(Box::new(UploadedFilePlugin)),
                _ => None,
            }
        })
        .collect();

    for map in &config.type_maps {
        plugins.push(Box::new(TypeMapPlugin::new(map.clone())));
    }
    plugins
}

pub(crate) fn attr_named(attrs: &[AttributeIr], names: &[&str]) -> bool {
    attrs.iter().any(|a| names.contains(&a.name.as_str()))
}

pub(crate) fn property_attrs<'a>(
    property: &'a PropertyIr,
    passed: &'a [AttributeIr],
) -> &'a [AttributeIr] {
    if passed.is_empty() {
        &property.attributes
    } else {
        passed
    }
}

pub(crate) fn to_snake_case(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    let mut out = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() {
            let prev = i.checked_sub(1).map(|j| chars[j]);
            let next = chars.get(i + 1).copied();
            let need_underscore = match prev {
                Some(p) if p.is_ascii_lowercase() || p.is_ascii_digit() => true,
                Some(p) if p.is_ascii_uppercase() => {
                    matches!(next, Some(n) if n.is_ascii_lowercase())
                }
                _ => false,
            };
            if need_underscore && !out.ends_with('_') {
                out.push('_');
            }
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

pub(crate) fn is_enum_writer(t: &dyn Transformer) -> bool {
    let dummy = EnumIr::new("Dummy", Vec::new(), "string");
    t.can_transform(TransformInput::Enum(&dummy), WriterContext::Enum, &[], &[])
}
