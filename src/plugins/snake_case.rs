use crate::ir::{TypeIr, WriterContext};
use crate::plugins::{to_snake_case, TransformInput, Transformer};

#[derive(Debug, Default)]
pub struct SnakeCasePlugin;

impl Transformer for SnakeCasePlugin {
    fn can_transform(
        &self,
        input: TransformInput<'_>,
        _context: WriterContext,
        _class_attributes: &[crate::ir::AttributeIr],
        _property_attributes: &[crate::ir::AttributeIr],
    ) -> bool {
        matches!(input, TransformInput::Property(_))
    }

    fn transform(
        &self,
        _input: TransformInput<'_>,
        _context: WriterContext,
        _class_attributes: &[crate::ir::AttributeIr],
        _property_attributes: &[crate::ir::AttributeIr],
        _inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String {
        String::new()
    }

    fn preprocess(&self, root: &mut crate::ir::RootIr) {
        for ns in root.namespaces.values_mut() {
            for schema in &mut ns.schemas {
                for prop in &mut schema.properties {
                    prop.name = to_snake_case(&prop.name);
                }
            }
        }
    }
}
