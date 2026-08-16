use crate::ir::{TypeIr, WriterContext};
use crate::plugins::{TransformInput, Transformer};

const COLLECTION_FQCN: &str = "Illuminate\\Support\\Collection";

#[derive(Debug, Default)]
pub struct CollectionPlugin;

impl CollectionPlugin {
    fn handles(ty: &TypeIr) -> bool {
        matches!(ty, TypeIr::Class { fqcn, .. } if fqcn == COLLECTION_FQCN)
    }

    fn item_type(ty: &TypeIr) -> Option<&TypeIr> {
        match ty {
            TypeIr::Class {
                type_parameters, ..
            } => {
                if type_parameters.len() >= 2 {
                    type_parameters.get(1)
                } else {
                    type_parameters.first()
                }
            }
            _ => None,
        }
    }
}

impl Transformer for CollectionPlugin {
    fn can_transform(
        &self,
        input: TransformInput<'_>,
        _context: WriterContext,
        _class_attributes: &[crate::ir::AttributeIr],
        _property_attributes: &[crate::ir::AttributeIr],
    ) -> bool {
        matches!(input, TransformInput::Type(ty) if Self::handles(ty))
    }

    fn transform(
        &self,
        input: TransformInput<'_>,
        context: WriterContext,
        _class_attributes: &[crate::ir::AttributeIr],
        _property_attributes: &[crate::ir::AttributeIr],
        inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String {
        let TransformInput::Type(ty) = input else {
            return "unknown".into();
        };
        if !Self::handles(ty) {
            return "unknown".into();
        }

        if let Some(item) = Self::item_type(ty) {
            let inner_type = inner(item, context);
            return match context {
                WriterContext::Schema => format!("S.Array({inner_type})"),
                _ => format!("readonly {inner_type}[]"),
            };
        }

        match context {
            WriterContext::Schema => "S.Array(S.Unknown)".into(),
            _ => "readonly unknown[]".into(),
        }
    }
}
