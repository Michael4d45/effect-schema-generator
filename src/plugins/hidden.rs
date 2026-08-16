use crate::ir::{AttributeIr, PropertyIr, TypeIr, WriterContext};
use crate::plugins::{attr_named, property_attrs, TransformInput, Transformer};

const HIDDEN: &str = "EffectSchemaGenerator\\Attributes\\Hidden";
const COMPUTED: &str = "Spatie\\LaravelData\\Attributes\\Computed";
const EXCLUDE: &[&str] = &[HIDDEN, COMPUTED];

#[derive(Debug, Default)]
pub struct HiddenPlugin;

impl HiddenPlugin {
    fn should_exclude(
        property: &PropertyIr,
        class_attributes: &[AttributeIr],
        property_attributes: &[AttributeIr],
    ) -> bool {
        attr_named(property_attrs(property, property_attributes), EXCLUDE)
            || attr_named(class_attributes, EXCLUDE)
    }
}

impl Transformer for HiddenPlugin {
    fn can_transform(
        &self,
        input: TransformInput<'_>,
        _context: WriterContext,
        class_attributes: &[AttributeIr],
        property_attributes: &[AttributeIr],
    ) -> bool {
        match input {
            TransformInput::Property(property) => {
                Self::should_exclude(property, class_attributes, property_attributes)
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
                    if Self::should_exclude(prop, &class_attrs, &prop_attrs) {
                        prop.hidden = true;
                    }
                }
            }
        }
    }
}
