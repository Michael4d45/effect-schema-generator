use crate::ir::{TypeIr, WriterContext};
use crate::plugins::{TransformInput, Transformer};

const UPLOADED_FILE_FQCN: &str = "Illuminate\\Http\\UploadedFile";

pub const UPLOADED_FILE_TS: &str = r#"import { Schema as S } from 'effect';

/** Browser File type for multipart uploads; matches Laravel Illuminate\Http\UploadedFile on the client. */
export type UploadedFile = File;

export const UploadedFileSchema: S.Schema<UploadedFile> =
    S.Any as S.Schema<UploadedFile>;
"#;

#[derive(Debug, Default)]
pub struct UploadedFilePlugin;

impl UploadedFilePlugin {
    fn handles(ty: &TypeIr) -> bool {
        matches!(ty, TypeIr::Class { fqcn, .. } if fqcn == UPLOADED_FILE_FQCN)
    }
}

impl Transformer for UploadedFilePlugin {
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
            WriterContext::Interface | WriterContext::EncodedInterface | WriterContext::Enum => {
                "UploadedFile".into()
            }
            WriterContext::Schema => "S.suspend(() => UploadedFileSchema)".into(),
        }
    }

    fn provides_file(&self) -> bool {
        true
    }

    fn file_path(&self) -> Option<String> {
        Some("Illuminate/UploadedFile.ts".into())
    }

    fn file_content(&self) -> Option<String> {
        Some(UPLOADED_FILE_TS.to_string())
    }
}
