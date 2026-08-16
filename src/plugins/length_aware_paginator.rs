use crate::ir::{TypeIr, WriterContext};
use crate::plugins::{TransformInput, Transformer};

const PAGINATOR_FQCN: &str = "Illuminate\\Pagination\\LengthAwarePaginator";

pub const PAGINATION_TS: &str = r#"import { Schema as S } from 'effect';

export interface PaginationLink {
    readonly url: string | null;
    readonly label: string;
    readonly page?: number | null | undefined;
    readonly active: boolean;
}

export const PaginationLinkSchema = S.Struct({
    url: S.NullOr(S.String),
    label: S.String,
    page: S.optional(S.NullOr(S.Number)),
    active: S.Boolean,
});

export interface PaginationMeta {
    readonly current_page: number;
    readonly first_page_url: string;
    readonly from: number | null;
    readonly last_page: number;
    readonly last_page_url: string;
    readonly next_page_url: string | null;
    readonly path: string;
    readonly per_page: number;
    readonly prev_page_url: string | null;
    readonly to: number | null;
    readonly total: number;
}

export const PaginationMetaSchema = S.Struct({
    current_page: S.Number,
    first_page_url: S.String,
    from: S.NullOr(S.Number),
    last_page: S.Number,
    last_page_url: S.String,
    next_page_url: S.NullOr(S.String),
    path: S.String,
    per_page: S.Number,
    prev_page_url: S.NullOr(S.String),
    to: S.NullOr(S.Number),
    total: S.Number,
});

export interface LengthAwarePaginator<T extends object> {
    readonly data: readonly T[];
    readonly links: readonly PaginationLink[];
    readonly meta: PaginationMeta;
}

export const LengthAwarePaginatorSchema = <A extends S.Schema.Any>(item: A) =>
    S.Struct({
        data: S.Array(item),
        links: S.Array(PaginationLinkSchema),
        meta: PaginationMetaSchema,
    });
"#;

#[derive(Debug, Default)]
pub struct LengthAwarePaginatorPlugin;

impl LengthAwarePaginatorPlugin {
    fn handles(ty: &TypeIr) -> bool {
        matches!(ty, TypeIr::Class { fqcn, .. } if fqcn == PAGINATOR_FQCN)
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

    fn is_primitive(ty: &TypeIr) -> bool {
        matches!(
            ty,
            TypeIr::String
                | TypeIr::Int
                | TypeIr::Float
                | TypeIr::Bool
                | TypeIr::Array { .. }
                | TypeIr::Union { .. }
                | TypeIr::Nullable { .. }
        )
    }
}

impl Transformer for LengthAwarePaginatorPlugin {
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
            if Self::is_primitive(item) {
                let schema_type = inner(item, WriterContext::Schema);
                return format!("LengthAwarePaginatorSchema({schema_type})");
            }
            let inner_type = inner(item, context);
            return match context {
                WriterContext::Schema => {
                    format!("LengthAwarePaginatorSchema({inner_type})")
                }
                _ => format!("LengthAwarePaginator<{inner_type}>"),
            };
        }

        match context {
            WriterContext::Schema => "LengthAwarePaginatorSchema(S.Unknown)".into(),
            _ => "LengthAwarePaginator<unknown>".into(),
        }
    }

    fn provides_file(&self) -> bool {
        true
    }

    fn file_path(&self) -> Option<String> {
        Some("Illuminate/Pagination.ts".into())
    }

    fn file_content(&self) -> Option<String> {
        Some(PAGINATION_TS.to_string())
    }
}
