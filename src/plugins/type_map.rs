use crate::config::TypeMap;
use crate::ir::{TypeIr, WriterContext};
use crate::plugins::{TransformInput, Transformer};

#[derive(Debug, Clone)]
pub struct TypeMapPlugin {
    map: TypeMap,
}

impl TypeMapPlugin {
    pub fn new(map: TypeMap) -> Self {
        Self { map }
    }

    fn handles(&self, ty: &TypeIr) -> bool {
        match ty {
            TypeIr::Class { fqcn, .. } => self
                .map
                .match_fqcns
                .iter()
                .any(|candidate| candidate == fqcn),
            _ => false,
        }
    }
}

impl Transformer for TypeMapPlugin {
    fn can_transform(
        &self,
        input: TransformInput<'_>,
        _context: WriterContext,
        _class_attributes: &[crate::ir::AttributeIr],
        _property_attributes: &[crate::ir::AttributeIr],
    ) -> bool {
        matches!(input, TransformInput::Type(ty) if self.handles(ty))
    }

    fn transform(
        &self,
        input: TransformInput<'_>,
        context: WriterContext,
        _class_attributes: &[crate::ir::AttributeIr],
        _property_attributes: &[crate::ir::AttributeIr],
        _inner: &dyn Fn(&TypeIr, WriterContext) -> String,
    ) -> String {
        if !matches!(input, TransformInput::Type(ty) if self.handles(ty)) {
            return "unknown".into();
        }
        match context {
            WriterContext::Interface => self.map.interface.clone(),
            WriterContext::EncodedInterface => self
                .map
                .encoded
                .clone()
                .unwrap_or_else(|| self.map.interface.clone()),
            WriterContext::Schema => self.map.schema.clone(),
            WriterContext::Enum => self
                .map
                .enum_type
                .clone()
                .or_else(|| self.map.encoded.clone())
                .unwrap_or_else(|| self.map.interface.clone()),
        }
    }

    fn provides_file(&self) -> bool {
        self.map.file_path.is_some() && (self.map.file_content.is_some() || self.map.file.is_some())
    }

    fn file_path(&self) -> Option<String> {
        self.map.file_path.clone()
    }

    fn file_content(&self) -> Option<String> {
        self.map.file_content.clone()
    }
}
