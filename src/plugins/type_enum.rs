use crate::ir::{EnumCaseIr, EnumIr, TypeIr, WriterContext};
use crate::plugins::{TransformInput, Transformer};

#[derive(Debug, Default)]
pub struct TypeEnumWriter;

impl TypeEnumWriter {
    pub fn write_enum(enum_ir: &EnumIr) -> String {
        let values: Vec<String> = enum_ir.cases.iter().map(literal_value).collect();
        format!("export type {} = {};", enum_ir.name, values.join(" | "))
    }
}

impl Transformer for TypeEnumWriter {
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

pub(crate) fn case_raw_value(case: &EnumCaseIr) -> serde_json::Value {
    case.value
        .clone()
        .unwrap_or_else(|| serde_json::Value::String(case.name.clone()))
}

pub(crate) fn literal_value(case: &EnumCaseIr) -> String {
    match case_raw_value(case) {
        serde_json::Value::String(s) => format!("\"{s}\""),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        other => other.to_string(),
    }
}
