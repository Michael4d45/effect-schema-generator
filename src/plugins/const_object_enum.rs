use crate::ir::{EnumIr, TypeIr, WriterContext};
use crate::plugins::type_enum::case_raw_value;
use crate::plugins::{TransformInput, Transformer};

#[derive(Debug, Default)]
pub struct ConstObjectEnumWriter;

impl ConstObjectEnumWriter {
    pub fn write_enum(enum_ir: &EnumIr) -> String {
        let entries: Vec<String> = enum_ir
            .cases
            .iter()
            .map(|case| {
                let value = match case_raw_value(case) {
                    serde_json::Value::String(s) => format!("'{s}'"),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    other => other.to_string(),
                };
                format!("    {}: {}", case.name, value)
            })
            .collect();
        format!(
            "export const {} = {{\n{}\n}} as const;",
            enum_ir.name,
            entries.join(",\n")
        )
    }
}

impl Transformer for ConstObjectEnumWriter {
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
