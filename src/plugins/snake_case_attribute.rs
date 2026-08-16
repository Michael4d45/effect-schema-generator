use crate::ir::{AttributeIr, PropertyIr, TypeIr, WriterContext};
use crate::plugins::{attr_named, property_attrs, to_snake_case, TransformInput, Transformer};

const SNAKE_CASE_ATTR: &str = "EffectSchemaGenerator\\Attributes\\SnakeCase";

#[derive(Debug, Default)]
pub struct SnakeCaseAttributePlugin;

impl SnakeCaseAttributePlugin {
    fn has_attr(
        property: &PropertyIr,
        class_attributes: &[AttributeIr],
        property_attributes: &[AttributeIr],
    ) -> bool {
        attr_named(
            property_attrs(property, property_attributes),
            &[SNAKE_CASE_ATTR],
        ) || attr_named(class_attributes, &[SNAKE_CASE_ATTR])
    }
}

impl Transformer for SnakeCaseAttributePlugin {
    fn can_transform(
        &self,
        input: TransformInput<'_>,
        _context: WriterContext,
        class_attributes: &[AttributeIr],
        property_attributes: &[AttributeIr],
    ) -> bool {
        match input {
            TransformInput::Property(property) => {
                Self::has_attr(property, class_attributes, property_attributes)
            }
            _ => false,
        }
    }

    fn transform(
        &self,
        _input: TransformInput<'_>,
        _context: WriterContext,
        _class_attributes: &[AttributeIr],
        _property_attributes: &[AttributeIr],
        _inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String {
        String::new()
    }

    fn preprocess(&self, root: &mut crate::ir::RootIr) {
        for ns in root.namespaces.values_mut() {
            for schema in &mut ns.schemas {
                let class_attrs = schema.class_attributes.clone();
                for prop in &mut schema.properties {
                    let prop_attrs = prop.attributes.clone();
                    if Self::has_attr(prop, &class_attrs, &prop_attrs) {
                        prop.name = to_snake_case(&prop.name);
                    }
                }
            }
        }
    }
}
