use crate::ir::{EnumIr, TypeIr, WriterContext};
use crate::plugins::type_enum::literal_value;
use crate::plugins::{TransformInput, Transformer};

#[derive(Debug, Default)]
pub struct EffectSchemaEnumWriter;

impl EffectSchemaEnumWriter {
    pub fn write_enum(enum_ir: &EnumIr) -> String {
        let literals: Vec<String> = enum_ir
            .cases
            .iter()
            .map(|case| format!("S.Literal({})", literal_value(case)))
            .collect();
        format!(
            "export const {}Schema = S.Union({});",
            enum_ir.name,
            literals.join(", ")
        )
    }
}

impl Transformer for EffectSchemaEnumWriter {
    fn can_transform(
        &self,
        input: TransformInput<'_>,
        context: WriterContext,
        _class_attributes: &[crate::ir::AttributeIr],
        _property_attributes: &[crate::ir::AttributeIr],
    ) -> bool {
        matches!(input, TransformInput::Enum(_)) && context == WriterContext::Enum
    }

    fn transform(
        &self,
        input: TransformInput<'_>,
        context: WriterContext,
        _class_attributes: &[crate::ir::AttributeIr],
        _property_attributes: &[crate::ir::AttributeIr],
        _inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String {
        match (input, context) {
            (TransformInput::Enum(enum_ir), WriterContext::Enum) => Self::write_enum(enum_ir),
            _ => String::new(),
        }
    }
}
