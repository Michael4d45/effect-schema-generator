use effect_schema_generator::config::Config;
use effect_schema_generator::ir::{
    AttributeIr, NamespaceIr, PropertyIr, RootIr, SchemaIr, TypeIr, WriterContext,
};
use effect_schema_generator::plugins::{
    load_plugins, CollectionPlugin, DatePlugin, HiddenPlugin, LazyOptionalPlugin,
    LengthAwarePaginatorPlugin, SnakeCaseAttributePlugin, SnakeCasePlugin, TransformInput,
    Transformer, TypeMapPlugin, UploadedFilePlugin,
};
use effect_schema_generator::writer::default_transform;
use effect_schema_generator::TypeMap;

fn inner(ty: &TypeIr, ctx: WriterContext) -> String {
    default_transform(ty, ctx)
}

fn can_type(p: &dyn Transformer, ty: &TypeIr, ctx: WriterContext) -> bool {
    p.can_transform(TransformInput::Type(ty), ctx, &[], &[])
}

fn xform_type(p: &dyn Transformer, ty: &TypeIr, ctx: WriterContext) -> String {
    p.transform(TransformInput::Type(ty), ctx, &[], &[], &inner)
}

fn can_prop(
    p: &dyn Transformer,
    prop: &PropertyIr,
    class_attrs: &[AttributeIr],
    ctx: WriterContext,
) -> bool {
    p.can_transform(
        TransformInput::Property(prop),
        ctx,
        class_attrs,
        &prop.attributes,
    )
}

fn schema_with(name: &str, properties: Vec<PropertyIr>, class_attrs: Vec<AttributeIr>) -> RootIr {
    let mut ns = NamespaceIr::new("App\\Data");
    let mut schema = SchemaIr::new(name, properties);
    schema.class_attributes = class_attrs;
    ns.schemas.push(schema);
    let mut root = RootIr::default();
    root.namespaces.insert(ns.namespace.clone(), ns);
    root
}

// --- DatePlugin ---

#[test]
fn date_can_transform_known_date_types() {
    let plugin = DatePlugin;
    for fqcn in [
        "Carbon\\Carbon",
        "Illuminate\\Support\\Carbon",
        "DateTime",
        "DateTimeImmutable",
    ] {
        let ty = TypeIr::class_ref(fqcn);
        assert!(
            can_type(&plugin, &ty, WriterContext::Interface),
            "expected {fqcn} to be a date"
        );
    }
}

#[test]
fn date_cannot_transform_non_date_types() {
    let plugin = DatePlugin;
    let ty = TypeIr::class_ref("App\\Data\\UserData");
    assert!(!can_type(&plugin, &ty, WriterContext::Interface));
    assert!(!can_type(
        &plugin,
        &TypeIr::String,
        WriterContext::Interface
    ));
}

#[test]
fn date_transforms_by_context() {
    let plugin = DatePlugin;
    let ty = TypeIr::class_ref("Carbon\\Carbon");
    assert_eq!(xform_type(&plugin, &ty, WriterContext::Interface), "Date");
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::EncodedInterface),
        "string"
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Schema),
        "S.DateFromString"
    );
    assert_eq!(xform_type(&plugin, &ty, WriterContext::Enum), "string");
}

#[test]
fn date_transforms_all_date_fqcns() {
    let plugin = DatePlugin;
    for fqcn in [
        "Carbon\\Carbon",
        "Illuminate\\Support\\Carbon",
        "DateTime",
        "DateTimeImmutable",
    ] {
        let ty = TypeIr::class_ref(fqcn);
        assert_eq!(xform_type(&plugin, &ty, WriterContext::Interface), "Date");
        assert_eq!(
            xform_type(&plugin, &ty, WriterContext::EncodedInterface),
            "string"
        );
        assert_eq!(
            xform_type(&plugin, &ty, WriterContext::Schema),
            "S.DateFromString"
        );
    }
}

#[test]
fn date_does_not_provide_a_file() {
    let plugin = DatePlugin;
    assert!(!plugin.provides_file());
    assert!(plugin.file_path().is_none());
    assert!(plugin.file_content().is_none());
}

// --- CollectionPlugin ---

#[test]
fn collection_can_transform_collection_type() {
    let plugin = CollectionPlugin;
    let ty = TypeIr::class_ref("Illuminate\\Support\\Collection");
    assert!(can_type(&plugin, &ty, WriterContext::Interface));
}

#[test]
fn collection_cannot_transform_other_classes() {
    let plugin = CollectionPlugin;
    let ty = TypeIr::class_ref("App\\Data\\UserData");
    assert!(!can_type(&plugin, &ty, WriterContext::Interface));
}

#[test]
fn collection_one_type_param_interface() {
    let plugin = CollectionPlugin;
    let ty = TypeIr::class_ref_params(
        "Illuminate\\Support\\Collection",
        vec![TypeIr::class_ref("App\\Data\\UserData")],
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "readonly UserData[]"
    );
}

#[test]
fn collection_two_type_params_uses_second() {
    let plugin = CollectionPlugin;
    let ty = TypeIr::class_ref_params(
        "Illuminate\\Support\\Collection",
        vec![TypeIr::String, TypeIr::class_ref("App\\Data\\UserData")],
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "readonly UserData[]"
    );
}

#[test]
fn collection_primitive_item_types() {
    let plugin = CollectionPlugin;
    let strings = TypeIr::class_ref_params(
        "Illuminate\\Support\\Collection",
        vec![TypeIr::String, TypeIr::String],
    );
    assert_eq!(
        xform_type(&plugin, &strings, WriterContext::Interface),
        "readonly string[]"
    );
    let ints = TypeIr::class_ref_params(
        "Illuminate\\Support\\Collection",
        vec![TypeIr::String, TypeIr::Int],
    );
    assert_eq!(
        xform_type(&plugin, &ints, WriterContext::Interface),
        "readonly number[]"
    );
}

#[test]
fn collection_schema_context() {
    let plugin = CollectionPlugin;
    let primitive = TypeIr::class_ref_params(
        "Illuminate\\Support\\Collection",
        vec![TypeIr::String, TypeIr::String],
    );
    assert_eq!(
        xform_type(&plugin, &primitive, WriterContext::Schema),
        "S.Array(S.String)"
    );
    let class_items = TypeIr::class_ref_params(
        "Illuminate\\Support\\Collection",
        vec![TypeIr::String, TypeIr::class_ref("App\\Data\\UserData")],
    );
    assert_eq!(
        xform_type(&plugin, &class_items, WriterContext::Schema),
        "S.Array(S.suspend(() => UserDataSchema))"
    );
}

#[test]
fn collection_without_type_params() {
    let plugin = CollectionPlugin;
    let ty = TypeIr::class_ref("Illuminate\\Support\\Collection");
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "readonly unknown[]"
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Schema),
        "S.Array(S.Unknown)"
    );
}

// --- HiddenPlugin ---

#[test]
fn hidden_can_transform_hidden_attribute() {
    let plugin = HiddenPlugin;
    let prop = PropertyIr {
        attributes: vec![AttributeIr::new(
            "EffectSchemaGenerator\\Attributes\\Hidden",
        )],
        ..PropertyIr::new("secret", TypeIr::String)
    };
    assert!(can_prop(&plugin, &prop, &[], WriterContext::Interface));
}

#[test]
fn hidden_cannot_transform_plain_property() {
    let plugin = HiddenPlugin;
    let prop = PropertyIr::new("name", TypeIr::String);
    assert!(!can_prop(&plugin, &prop, &[], WriterContext::Interface));
}

#[test]
fn hidden_can_transform_spatie_computed() {
    let plugin = HiddenPlugin;
    let prop = PropertyIr {
        attributes: vec![AttributeIr::new(
            "Spatie\\LaravelData\\Attributes\\Computed",
        )],
        ..PropertyIr::new("numerator", TypeIr::Int)
    };
    assert!(can_prop(&plugin, &prop, &[], WriterContext::Interface));
}

#[test]
fn hidden_can_transform_class_level_hidden() {
    let plugin = HiddenPlugin;
    let prop = PropertyIr::new("internal", TypeIr::String);
    let class_attrs = vec![AttributeIr::new(
        "EffectSchemaGenerator\\Attributes\\Hidden",
    )];
    assert!(can_prop(
        &plugin,
        &prop,
        &class_attrs,
        WriterContext::Interface
    ));
}

#[test]
fn hidden_cannot_transform_type_ir() {
    let plugin = HiddenPlugin;
    assert!(!can_type(
        &plugin,
        &TypeIr::String,
        WriterContext::Interface
    ));
}

#[test]
fn hidden_preprocess_sets_hidden_flag() {
    let plugin = HiddenPlugin;
    let mut root = schema_with(
        "UserData",
        vec![
            PropertyIr::new("name", TypeIr::String),
            PropertyIr {
                attributes: vec![AttributeIr::new(
                    "EffectSchemaGenerator\\Attributes\\Hidden",
                )],
                ..PropertyIr::new("secret", TypeIr::String)
            },
        ],
        vec![],
    );
    plugin.preprocess(&mut root);
    let props = &root.namespaces["App\\Data"].schemas[0].properties;
    assert!(!props[0].hidden);
    assert!(props[1].hidden);
}

#[test]
fn hidden_does_not_provide_a_file() {
    let plugin = HiddenPlugin;
    assert!(!plugin.provides_file());
    assert!(plugin.file_path().is_none());
    assert!(plugin.file_content().is_none());
}

// --- LazyOptionalPlugin ---

#[test]
fn lazy_can_transform_standalone_lazy_and_optional_types() {
    let plugin = LazyOptionalPlugin;
    assert!(can_type(
        &plugin,
        &TypeIr::class_ref("Spatie\\LaravelData\\Lazy"),
        WriterContext::Interface
    ));
    assert!(can_type(
        &plugin,
        &TypeIr::class_ref("Spatie\\LaravelData\\Optional"),
        WriterContext::Interface
    ));
}

#[test]
fn lazy_can_transform_union_containing_lazy() {
    let plugin = LazyOptionalPlugin;
    let ty = TypeIr::Union {
        types: vec![
            TypeIr::class_ref("Illuminate\\Support\\Collection"),
            TypeIr::class_ref("Spatie\\LaravelData\\Lazy"),
        ],
    };
    assert!(can_type(&plugin, &ty, WriterContext::Interface));
}

#[test]
fn lazy_cannot_transform_unrelated_types() {
    let plugin = LazyOptionalPlugin;
    assert!(!can_type(
        &plugin,
        &TypeIr::class_ref("App\\Data\\UserData"),
        WriterContext::Interface
    ));
    let ty = TypeIr::Union {
        types: vec![
            TypeIr::class_ref("Illuminate\\Support\\Collection"),
            TypeIr::String,
        ],
    };
    assert!(!can_type(&plugin, &ty, WriterContext::Interface));
}

#[test]
fn lazy_preprocess_marks_property_optional() {
    let plugin = LazyOptionalPlugin;
    let mut root = schema_with(
        "ResponseData",
        vec![PropertyIr::new(
            "lazyField",
            TypeIr::class_ref("Spatie\\LaravelData\\Lazy"),
        )],
        vec![],
    );
    assert!(can_prop(
        &plugin,
        &root.namespaces["App\\Data"].schemas[0].properties[0],
        &[],
        WriterContext::Interface
    ));
    plugin.preprocess(&mut root);
    let prop = &root.namespaces["App\\Data"].schemas[0].properties[0];
    assert!(prop.optional);
    assert_eq!(prop.ty, TypeIr::Unknown);
}

#[test]
fn lazy_transforms_type_param() {
    let plugin = LazyOptionalPlugin;
    let ty = TypeIr::class_ref_params(
        "Spatie\\LaravelData\\Lazy",
        vec![TypeIr::class_ref("App\\Data\\UserData")],
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "UserData"
    );
}

#[test]
fn lazy_without_type_params_is_unknown() {
    let plugin = LazyOptionalPlugin;
    let ty = TypeIr::class_ref("Spatie\\LaravelData\\Lazy");
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "unknown"
    );
}

#[test]
fn lazy_strips_lazy_from_union() {
    let plugin = LazyOptionalPlugin;
    let ty = TypeIr::Union {
        types: vec![
            TypeIr::class_ref("Illuminate\\Support\\Collection"),
            TypeIr::class_ref("Spatie\\LaravelData\\Lazy"),
        ],
    };
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "Collection"
    );
}

#[test]
fn lazy_strips_lazy_from_multi_union() {
    let plugin = LazyOptionalPlugin;
    let ty = TypeIr::Union {
        types: vec![
            TypeIr::class_ref("Illuminate\\Support\\Collection"),
            TypeIr::String,
            TypeIr::class_ref("Spatie\\LaravelData\\Lazy"),
        ],
    };
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "Collection | string"
    );
}

#[test]
fn lazy_union_of_only_lazy_is_unknown() {
    let plugin = LazyOptionalPlugin;
    let ty = TypeIr::Union {
        types: vec![TypeIr::class_ref("Spatie\\LaravelData\\Lazy")],
    };
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "unknown"
    );
}

#[test]
fn lazy_preprocess_optional_attribute_on_class() {
    let plugin = LazyOptionalPlugin;
    let mut root = schema_with(
        "OptionalClassData",
        vec![
            PropertyIr::new("name", TypeIr::String),
            PropertyIr::new("age", TypeIr::Int),
        ],
        vec![AttributeIr::new(
            "EffectSchemaGenerator\\Attributes\\Optional",
        )],
    );
    plugin.preprocess(&mut root);
    for prop in &root.namespaces["App\\Data"].schemas[0].properties {
        assert!(prop.optional, "{} should be optional", prop.name);
    }
}

#[test]
fn lazy_preprocess_strips_lazy_from_union_property() {
    let plugin = LazyOptionalPlugin;
    let mut root = schema_with(
        "ResponseData",
        vec![PropertyIr::new(
            "participants",
            TypeIr::Union {
                types: vec![
                    TypeIr::class_ref("Illuminate\\Support\\Collection"),
                    TypeIr::class_ref("Spatie\\LaravelData\\Lazy"),
                ],
            },
        )],
        vec![],
    );
    plugin.preprocess(&mut root);
    let prop = &root.namespaces["App\\Data"].schemas[0].properties[0];
    assert!(prop.optional);
    assert_eq!(
        prop.ty,
        TypeIr::class_ref("Illuminate\\Support\\Collection")
    );
}

// --- SnakeCasePlugin ---

#[test]
fn snake_case_can_transform_any_property() {
    let plugin = SnakeCasePlugin;
    let prop = PropertyIr::new("firstName", TypeIr::String);
    assert!(can_prop(&plugin, &prop, &[], WriterContext::Interface));
    assert!(!can_type(
        &plugin,
        &TypeIr::String,
        WriterContext::Interface
    ));
}

#[test]
fn snake_case_preprocess_renames_properties() {
    let plugin = SnakeCasePlugin;
    let mut root = schema_with(
        "UserData",
        vec![
            PropertyIr::new("firstName", TypeIr::String),
            PropertyIr::new("myLongPropertyName", TypeIr::String),
            PropertyIr::new("already_snake", TypeIr::String),
            PropertyIr::new("name", TypeIr::String),
            PropertyIr::new("getHTTPResponse", TypeIr::String),
        ],
        vec![],
    );
    plugin.preprocess(&mut root);
    let names: Vec<_> = root.namespaces["App\\Data"].schemas[0]
        .properties
        .iter()
        .map(|p| p.name.as_str())
        .collect();
    assert_eq!(
        names,
        [
            "first_name",
            "my_long_property_name",
            "already_snake",
            "name",
            "get_http_response"
        ]
    );
}

#[test]
fn snake_case_does_not_provide_a_file() {
    let plugin = SnakeCasePlugin;
    assert!(!plugin.provides_file());
}

// --- SnakeCaseAttributePlugin ---

#[test]
fn snake_case_attr_requires_attribute() {
    let plugin = SnakeCaseAttributePlugin;
    let with_attr = PropertyIr {
        attributes: vec![AttributeIr::new(
            "EffectSchemaGenerator\\Attributes\\SnakeCase",
        )],
        ..PropertyIr::new("firstName", TypeIr::String)
    };
    assert!(can_prop(&plugin, &with_attr, &[], WriterContext::Interface));
    let without = PropertyIr::new("firstName", TypeIr::String);
    assert!(!can_prop(&plugin, &without, &[], WriterContext::Interface));
}

#[test]
fn snake_case_attr_class_level() {
    let plugin = SnakeCaseAttributePlugin;
    let prop = PropertyIr::new("firstName", TypeIr::String);
    let class_attrs = vec![AttributeIr::new(
        "EffectSchemaGenerator\\Attributes\\SnakeCase",
    )];
    assert!(can_prop(
        &plugin,
        &prop,
        &class_attrs,
        WriterContext::Interface
    ));
}

#[test]
fn snake_case_attr_preprocess_only_matching_properties() {
    let plugin = SnakeCaseAttributePlugin;
    let attr = AttributeIr::new("EffectSchemaGenerator\\Attributes\\SnakeCase");
    let mut root = schema_with(
        "UserData",
        vec![
            PropertyIr {
                attributes: vec![attr],
                ..PropertyIr::new("firstName", TypeIr::String)
            },
            PropertyIr::new("lastName", TypeIr::String),
        ],
        vec![],
    );
    plugin.preprocess(&mut root);
    let props = &root.namespaces["App\\Data"].schemas[0].properties;
    assert_eq!(props[0].name, "first_name");
    assert_eq!(props[1].name, "lastName");
}

// --- LengthAwarePaginatorPlugin ---

#[test]
fn paginator_can_transform() {
    let plugin = LengthAwarePaginatorPlugin;
    let ty = TypeIr::class_ref("Illuminate\\Pagination\\LengthAwarePaginator");
    assert!(can_type(&plugin, &ty, WriterContext::Interface));
    assert!(!can_type(
        &plugin,
        &TypeIr::class_ref("App\\Data\\UserData"),
        WriterContext::Interface
    ));
}

#[test]
fn paginator_one_type_param() {
    let plugin = LengthAwarePaginatorPlugin;
    let ty = TypeIr::class_ref_params(
        "Illuminate\\Pagination\\LengthAwarePaginator",
        vec![TypeIr::class_ref("App\\Data\\UserData")],
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "LengthAwarePaginator<UserData>"
    );
}

#[test]
fn paginator_two_type_params_uses_second() {
    let plugin = LengthAwarePaginatorPlugin;
    let ty = TypeIr::class_ref_params(
        "Illuminate\\Pagination\\LengthAwarePaginator",
        vec![TypeIr::String, TypeIr::class_ref("App\\Data\\UserData")],
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "LengthAwarePaginator<UserData>"
    );
}

#[test]
fn paginator_primitive_uses_schema_helper() {
    let plugin = LengthAwarePaginatorPlugin;
    let ty = TypeIr::class_ref_params(
        "Illuminate\\Pagination\\LengthAwarePaginator",
        vec![TypeIr::String, TypeIr::String],
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "LengthAwarePaginatorSchema(S.String)"
    );
}

#[test]
fn paginator_without_params() {
    let plugin = LengthAwarePaginatorPlugin;
    let ty = TypeIr::class_ref("Illuminate\\Pagination\\LengthAwarePaginator");
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "LengthAwarePaginator<unknown>"
    );
}

#[test]
fn paginator_schema_class_item() {
    let plugin = LengthAwarePaginatorPlugin;
    let ty = TypeIr::class_ref_params(
        "Illuminate\\Pagination\\LengthAwarePaginator",
        vec![TypeIr::String, TypeIr::class_ref("App\\Data\\UserData")],
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Schema),
        "LengthAwarePaginatorSchema(S.suspend(() => UserDataSchema))"
    );
}

#[test]
fn paginator_provides_file() {
    let plugin = LengthAwarePaginatorPlugin;
    assert!(plugin.provides_file());
    assert_eq!(
        plugin.file_path().as_deref(),
        Some("Illuminate/Pagination.ts")
    );
    let content = plugin.file_content().expect("file content");
    assert!(content.contains("export interface PaginationLink"));
    assert!(content.contains("export interface PaginationMeta"));
    assert!(content.contains("export interface LengthAwarePaginator<T extends object>"));
    assert!(content.contains("url: string | null"));
    assert!(content.contains("data: readonly T[]"));
    assert!(content.contains("export const LengthAwarePaginatorSchema"));
    assert_eq!(content, LENGTH_AWARE_PAGINATOR_TS);
}

const LENGTH_AWARE_PAGINATOR_TS: &str = r#"import { Schema as S } from 'effect';

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

// --- load_plugins ---

#[test]
fn load_plugins_instantiates_from_config_names() {
    let cfg = Config {
        plugins: vec![
            "date".into(),
            "collection".into(),
            "hidden".into(),
            "lazy_optional".into(),
            "snake_case".into(),
            "snake_case_attribute".into(),
            "length_aware_paginator".into(),
            "type_enum".into(),
            "effect_schema_enum".into(),
            "const_object_enum".into(),
            "uploaded_file".into(),
        ],
        ..Config::default()
    };
    let plugins = load_plugins(&cfg);
    assert_eq!(plugins.len(), 11);

    let carbon = TypeIr::class_ref("Carbon\\Carbon");
    assert!(plugins.iter().any(|p| p.can_transform(
        TransformInput::Type(&carbon),
        WriterContext::Interface,
        &[],
        &[]
    )));
}

#[test]
fn uploaded_file_transforms_and_provides_helper() {
    let plugin = UploadedFilePlugin;
    let ty = TypeIr::class_ref("Illuminate\\Http\\UploadedFile");
    assert!(can_type(&plugin, &ty, WriterContext::Interface));
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "UploadedFile"
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::EncodedInterface),
        "UploadedFile"
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Schema),
        "S.suspend(() => UploadedFileSchema)"
    );
    assert!(plugin.provides_file());
    assert_eq!(
        plugin.file_path().as_deref(),
        Some("Illuminate/UploadedFile.ts")
    );
    assert!(plugin
        .file_content()
        .unwrap_or_default()
        .contains("export type UploadedFile = File"));
}

#[test]
fn load_plugins_skips_unknown_names() {
    let cfg = Config {
        plugins: vec!["nope".into(), "date".into()],
        ..Config::default()
    };
    let plugins = load_plugins(&cfg);
    assert_eq!(plugins.len(), 1);
}

#[test]
fn type_map_plugin_matches_uploaded_file() {
    let plugin = TypeMapPlugin::new(TypeMap {
        match_fqcns: vec!["Illuminate\\Http\\UploadedFile".into()],
        interface: "UploadedFile".into(),
        encoded: Some("UploadedFile".into()),
        schema: "S.suspend(() => UploadedFileSchema)".into(),
        enum_type: None,
        file_path: Some("Illuminate/UploadedFile.ts".into()),
        file_content: Some("export type UploadedFile = File;\n".into()),
        file: None,
    });
    let ty = TypeIr::class_ref("Illuminate\\Http\\UploadedFile");
    assert!(can_type(&plugin, &ty, WriterContext::Interface));
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Interface),
        "UploadedFile"
    );
    assert_eq!(
        xform_type(&plugin, &ty, WriterContext::Schema),
        "S.suspend(() => UploadedFileSchema)"
    );
    assert!(plugin.provides_file());
    assert_eq!(
        plugin.file_path().as_deref(),
        Some("Illuminate/UploadedFile.ts")
    );
}

#[test]
fn type_maps_load_from_toml_and_append_to_plugins() {
    let cfg = Config::from_toml_str(
        r#"
plugins = ["date"]

[[type_maps]]
match = "Illuminate\\Http\\UploadedFile"
interface = "UploadedFile"
encoded = "UploadedFile"
schema = "S.suspend(() => UploadedFileSchema)"
file_path = "Illuminate/UploadedFile.ts"
file_content = """
export type UploadedFile = File;
"""
"#,
    )
    .expect("parse type_maps toml");

    assert_eq!(cfg.type_maps.len(), 1);
    assert_eq!(
        cfg.type_maps[0].match_fqcns,
        vec!["Illuminate\\Http\\UploadedFile".to_string()]
    );

    let plugins = load_plugins(&cfg);
    assert_eq!(plugins.len(), 2);
    let ty = TypeIr::class_ref("Illuminate\\Http\\UploadedFile");
    assert!(plugins.iter().any(|p| p.can_transform(
        TransformInput::Type(&ty),
        WriterContext::Interface,
        &[],
        &[]
    )));
}
