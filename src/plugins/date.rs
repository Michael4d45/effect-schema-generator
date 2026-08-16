use crate::ir::{TypeIr, WriterContext};
use crate::plugins::{TransformInput, Transformer};

const DATE_FQCNS: &[&str] = &[
    "Illuminate\\Support\\Carbon",
    "Carbon\\Carbon",
    "DateTime",
    "DateTimeImmutable",
];

#[derive(Debug, Default)]
pub struct DatePlugin;

impl DatePlugin {
    fn handles(ty: &TypeIr) -> bool {
        match ty {
            TypeIr::Class { fqcn, .. } => DATE_FQCNS.contains(&fqcn.as_str()),
            _ => false,
        }
    }
}

impl Transformer for DatePlugin {
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
        _inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String {
        if !matches!(input, TransformInput::Type(ty) if Self::handles(ty)) {
            return "unknown".into();
        }
        match context {
            WriterContext::Interface => "Date".into(),
            WriterContext::EncodedInterface | WriterContext::Enum => "string".into(),
            WriterContext::Schema => "S.DateFromString".into(),
        }
    }
}
