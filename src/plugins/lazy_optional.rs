use crate::ir::{AttributeIr, PropertyIr, TypeIr, WriterContext};
use crate::plugins::{attr_named, property_attrs, TransformInput, Transformer};

const LAZY: &str = "Spatie\\LaravelData\\Lazy";
const OPTIONAL: &str = "Spatie\\LaravelData\\Optional";
const OPTIONAL_ATTRS: &[&str] = &[
    "Spatie\\LaravelData\\Attributes\\Optional",
    "EffectSchemaGenerator\\Attributes\\Optional",
];

#[derive(Debug, Default)]
pub struct LazyOptionalPlugin;

impl LazyOptionalPlugin {
    fn is_lazy_or_optional_class(ty: &TypeIr) -> bool {
        matches!(ty, TypeIr::Class { fqcn, .. } if fqcn == LAZY || fqcn == OPTIONAL)
    }

    fn handles(ty: &TypeIr) -> bool {
        if Self::is_lazy_or_optional_class(ty) {
            return true;
        }
        if let TypeIr::Union { types } = ty {
            return types.iter().any(Self::is_lazy_or_optional_class);
        }
        false
    }

    fn contains_lazy_at_top_level(ty: &TypeIr) -> bool {
        match ty {
            TypeIr::Union { types } => types.iter().any(|member| {
                let candidate = match member {
                    TypeIr::Nullable { inner } => inner.as_ref(),
                    other => other,
                };
                Self::is_lazy_or_optional_class(candidate)
            }),
            TypeIr::Class { .. } => Self::is_lazy_or_optional_class(ty),
            TypeIr::Nullable { inner } => Self::contains_lazy_at_top_level(inner),
            _ => false,
        }
    }

    fn has_optional_attribute(
        property: &PropertyIr,
        class_attributes: &[AttributeIr],
        property_attributes: &[AttributeIr],
    ) -> bool {
        attr_named(
            property_attrs(property, property_attributes),
            OPTIONAL_ATTRS,
        ) || attr_named(class_attributes, OPTIONAL_ATTRS)
    }

    fn remove_lazy_from_type(ty: &TypeIr) -> TypeIr {
        if Self::is_lazy_or_optional_class(ty) {
            return TypeIr::Unknown;
        }
        match ty {
            TypeIr::Nullable { inner } => TypeIr::Nullable {
                inner: Box::new(Self::remove_lazy_from_type(inner)),
            },
            TypeIr::Union { types } => Self::remove_lazy_from_union(types),
            other => other.clone(),
        }
    }

    fn remove_lazy_from_union(types: &[TypeIr]) -> TypeIr {
        let mut remaining = Vec::new();
        for union_type in types {
            let processed = Self::remove_lazy_from_type(union_type);
            if matches!(processed, TypeIr::Unknown) && !matches!(union_type, TypeIr::Unknown) {
                continue;
            }
            if matches!(&processed, TypeIr::Nullable { inner } if matches!(inner.as_ref(), TypeIr::Unknown))
            {
                continue;
            }
            remaining.push(processed);
        }
        match remaining.len() {
            0 => TypeIr::Unknown,
            1 => remaining.pop().unwrap(),
            _ => TypeIr::Union { types: remaining },
        }
    }

    fn transform_type(
        ty: &TypeIr,
        context: WriterContext,
        inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String {
        if Self::is_lazy_or_optional_class(ty) {
            if let TypeIr::Class {
                type_parameters, ..
            } = ty
            {
                if let Some(param) = type_parameters.first() {
                    return inner(param, context);
                }
            }
            return "unknown".into();
        }

        if let TypeIr::Union { types } = ty {
            let mut without = Vec::new();
            for union_type in types {
                if Self::is_lazy_or_optional_class(union_type) {
                    continue;
                }
                without.push(inner(union_type, context));
            }
            if without.is_empty() {
                return "unknown".into();
            }
            return without.join(" | ");
        }

        "unknown".into()
    }
}

impl Transformer for LazyOptionalPlugin {
    fn can_transform(
        &self,
        input: TransformInput<'_>,
        _context: WriterContext,
        class_attributes: &[AttributeIr],
        property_attributes: &[AttributeIr],
    ) -> bool {
        match input {
            TransformInput::Property(property) => {
                Self::contains_lazy_at_top_level(&property.ty)
                    || Self::has_optional_attribute(property, class_attributes, property_attributes)
            }
            TransformInput::Type(ty) => Self::handles(ty),
            _ => false,
        }
    }

    fn transform(
        &self,
        input: TransformInput<'_>,
        context: WriterContext,
        _class_attributes: &[AttributeIr],
        _property_attributes: &[AttributeIr],
        inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String {
        match input {
            TransformInput::Property(_) => String::new(),
            TransformInput::Type(ty) => Self::transform_type(ty, context, inner),
            _ => "unknown".into(),
        }
    }

    fn preprocess(&self, root: &mut crate::ir::RootIr) {
        for ns in root.namespaces.values_mut() {
            for schema in &mut ns.schemas {
                let class_attrs = schema.class_attributes.clone();
                for prop in &mut schema.properties {
                    let prop_attrs = prop.attributes.clone();
                    if Self::contains_lazy_at_top_level(&prop.ty)
                        || Self::has_optional_attribute(prop, &class_attrs, &prop_attrs)
                    {
                        prop.optional = true;
                    }
                    if Self::contains_lazy_at_top_level(&prop.ty) {
                        prop.ty = Self::remove_lazy_from_type(&prop.ty);
                    }
                }
            }
        }
    }
}
