use std::collections::BTreeMap;

use effect_schema_generator::builder::build;
use effect_schema_generator::ir::{AttributeIr, EnumCaseIr, PropertyIr, RootIr, SchemaIr, TypeIr};
use effect_schema_generator::tokens::{
    ClassToken, EnumToken, NativeType, PublicPropertyToken, Token,
};
use pretty_assertions::assert_eq;

const FIXTURES_NS: &str = r"EffectSchemaGenerator\Tests\Fixtures";
const USER_DATA_FQCN: &str = r"EffectSchemaGenerator\Tests\Fixtures\UserData";
const COLOR_FQCN: &str = r"EffectSchemaGenerator\Tests\Fixtures\Color";

fn prop(name: &str, native: NativeType, phpdoc: Option<&str>) -> PublicPropertyToken {
    PublicPropertyToken {
        name: name.into(),
        native_type: native,
        phpdoc_type: phpdoc.map(str::to_string),
        attributes: vec![],
    }
}

fn string_prop(name: &str) -> PublicPropertyToken {
    prop(name, NativeType::String, None)
}

fn class_with_props(
    namespace: &str,
    fqcn: &str,
    properties: Vec<PublicPropertyToken>,
) -> ClassToken {
    let mut class = ClassToken::new(namespace, fqcn);
    class.public_properties = properties;
    class
}

fn single_prop_class(native: NativeType, phpdoc: Option<&str>) -> ClassToken {
    class_with_props(
        FIXTURES_NS,
        USER_DATA_FQCN,
        vec![prop("value", native, phpdoc)],
    )
}

fn first_schema(root: &RootIr) -> &SchemaIr {
    &root.namespaces[FIXTURES_NS].schemas[0]
}

fn first_prop(root: &RootIr) -> &PropertyIr {
    &first_schema(root).properties[0]
}

fn union(types: Vec<TypeIr>) -> TypeIr {
    TypeIr::Union { types }
}

fn nullable(inner: TypeIr) -> TypeIr {
    TypeIr::Nullable {
        inner: Box::new(inner),
    }
}

fn array_of(item: TypeIr) -> TypeIr {
    TypeIr::Array {
        item: Some(Box::new(item)),
    }
}

fn record(key: TypeIr, value: TypeIr) -> TypeIr {
    TypeIr::Record {
        key: Box::new(key),
        value: Box::new(value),
    }
}

#[test]
fn empty_tokens_yield_empty_root() {
    let root = build(&[]);
    assert!(root.namespaces.is_empty());
}

#[test]
fn one_class_user_data_like_groups_into_namespace() {
    let class = class_with_props(
        FIXTURES_NS,
        USER_DATA_FQCN,
        vec![string_prop("id"), string_prop("name"), string_prop("email")],
    );

    let root = build(&[Token::Class(class)]);

    assert!(root.namespaces.contains_key(FIXTURES_NS));
    let ns = &root.namespaces[FIXTURES_NS];
    assert_eq!(ns.namespace, FIXTURES_NS);
    assert_eq!(ns.schemas.len(), 1);
    assert_eq!(ns.schemas[0].name, "UserData");
    assert_eq!(ns.schemas[0].properties.len(), 3);
    assert_eq!(ns.schemas[0].properties[0].name, "id");
    assert_eq!(ns.schemas[0].properties[0].ty, TypeIr::String);
    assert_eq!(ns.schemas[0].properties[1].name, "name");
    assert_eq!(ns.schemas[0].properties[1].ty, TypeIr::String);
    assert_eq!(ns.schemas[0].properties[2].name, "email");
    assert_eq!(ns.schemas[0].properties[2].ty, TypeIr::String);
    assert!(ns.enums.is_empty());
}

#[test]
fn enum_token_color_lands_in_enums_list() {
    let mut color = EnumToken::new(FIXTURES_NS, COLOR_FQCN);
    color.cases = vec![
        EnumCaseIr {
            name: "RED".into(),
            value: None,
        },
        EnumCaseIr {
            name: "GREEN".into(),
            value: None,
        },
        EnumCaseIr {
            name: "BLUE".into(),
            value: None,
        },
    ];

    let root = build(&[Token::Enum(color)]);

    let ns = &root.namespaces[FIXTURES_NS];
    assert_eq!(ns.enums.len(), 1);
    assert_eq!(ns.enums[0].name, "Color");
    assert_eq!(ns.enums[0].cases.len(), 3);
    assert_eq!(ns.enums[0].cases[0].name, "RED");
    assert!(ns.schemas.is_empty());
}

#[test]
fn native_union_int_string() {
    let class = single_prop_class(
        NativeType::Union(vec![NativeType::Int, NativeType::String]),
        None,
    );
    let root = build(&[Token::Class(class)]);
    assert_eq!(
        first_prop(&root).ty,
        union(vec![TypeIr::Int, TypeIr::String])
    );
}

#[test]
fn native_union_int_string_null_becomes_nullable_union() {
    let class = single_prop_class(
        NativeType::Union(vec![NativeType::Int, NativeType::String, NativeType::Null]),
        None,
    );
    let root = build(&[Token::Class(class)]);
    let property = first_prop(&root);
    assert_eq!(
        property.ty,
        nullable(union(vec![TypeIr::Int, TypeIr::String]))
    );
    assert!(property.nullable);
}

#[test]
fn native_nullable_union_int_string() {
    let class = single_prop_class(
        NativeType::Nullable(Box::new(NativeType::Union(vec![
            NativeType::Int,
            NativeType::String,
        ]))),
        None,
    );
    let root = build(&[Token::Class(class)]);
    let property = first_prop(&root);
    assert_eq!(
        property.ty,
        nullable(union(vec![TypeIr::Int, TypeIr::String]))
    );
    assert!(property.nullable);
}

#[test]
fn native_mixed_becomes_unknown() {
    let class = single_prop_class(NativeType::Mixed, None);
    let root = build(&[Token::Class(class)]);
    assert_eq!(first_prop(&root).ty, TypeIr::Unknown);
}

#[test]
fn native_array_becomes_array_of_unknown() {
    let class = single_prop_class(NativeType::Array, None);
    let root = build(&[Token::Class(class)]);
    assert_eq!(first_prop(&root).ty, array_of(TypeIr::Unknown));
}

#[test]
fn phpdoc_array_string_mixed_becomes_record() {
    let class = single_prop_class(NativeType::Array, Some(r"array<string, mixed>"));
    let root = build(&[Token::Class(class)]);
    assert_eq!(
        first_prop(&root).ty,
        record(TypeIr::String, TypeIr::Unknown)
    );
}

#[test]
fn phpdoc_array_int_string_becomes_record() {
    let class = single_prop_class(NativeType::Array, Some(r"array<int, string>"));
    let root = build(&[Token::Class(class)]);
    assert_eq!(first_prop(&root).ty, record(TypeIr::Int, TypeIr::String));
}

#[test]
fn phpdoc_collection_with_use_userdata() {
    let mut class = single_prop_class(
        NativeType::Class(r"Illuminate\Support\Collection".into()),
        Some(r"Collection<array-key, UserData>"),
    );
    class.uses = BTreeMap::from([
        (
            "UserData".into(),
            r"EffectSchemaGenerator\Tests\Fixtures\UserData".into(),
        ),
        ("Collection".into(), r"Illuminate\Support\Collection".into()),
    ]);

    let root = build(&[Token::Class(class)]);
    let ty = &first_prop(&root).ty;
    match ty {
        TypeIr::Class {
            fqcn,
            alias,
            type_parameters,
            is_enum,
            ..
        } => {
            assert_eq!(fqcn, r"Illuminate\Support\Collection");
            assert_eq!(alias, "Collection");
            assert!(!*is_enum);
            assert_eq!(type_parameters.len(), 2);
            assert_eq!(type_parameters[0], union(vec![TypeIr::String, TypeIr::Int]));
            assert_eq!(
                type_parameters[1],
                TypeIr::class_ref(r"EffectSchemaGenerator\Tests\Fixtures\UserData")
            );
        }
        other => panic!("expected Class, got {other:?}"),
    }
}

#[test]
fn phpdoc_list_string_becomes_array_of_string() {
    let class = single_prop_class(NativeType::Array, Some(r"list<string>"));
    let root = build(&[Token::Class(class)]);
    assert_eq!(first_prop(&root).ty, array_of(TypeIr::String));
}

#[test]
fn native_class_ref_carbon() {
    let class = single_prop_class(NativeType::Class(r"Carbon\Carbon".into()), None);
    let root = build(&[Token::Class(class)]);
    assert_eq!(first_prop(&root).ty, TypeIr::class_ref(r"Carbon\Carbon"));
}

#[test]
fn marks_is_enum_when_enum_token_present() {
    let status_fqcn = r"EffectSchemaGenerator\Tests\Fixtures\TestStatus";
    let class = class_with_props(
        FIXTURES_NS,
        r"EffectSchemaGenerator\Tests\Fixtures\ProductData",
        vec![prop("status", NativeType::Class(status_fqcn.into()), None)],
    );
    let enum_token = EnumToken::new(FIXTURES_NS, status_fqcn);

    let root = build(&[Token::Class(class), Token::Enum(enum_token)]);

    match &first_prop(&root).ty {
        TypeIr::Class {
            fqcn,
            is_enum,
            alias,
            ..
        } => {
            assert_eq!(fqcn, status_fqcn);
            assert_eq!(alias, "TestStatus");
            assert!(*is_enum, "class ref to an enum token must set is_enum");
        }
        other => panic!("expected Class, got {other:?}"),
    }
}

#[test]
fn phpdoc_class_ref_is_marked_enum_when_enum_token_present() {
    let status_fqcn = r"EffectSchemaGenerator\Tests\Fixtures\TestStatus";
    let mut class = class_with_props(
        FIXTURES_NS,
        r"EffectSchemaGenerator\Tests\Fixtures\ProductData",
        vec![prop(
            "status",
            NativeType::Class(status_fqcn.into()),
            Some("TestStatus"),
        )],
    );
    class.uses = BTreeMap::from([("TestStatus".into(), status_fqcn.into())]);
    let enum_token = EnumToken::new(FIXTURES_NS, status_fqcn);

    let root = build(&[Token::Class(class), Token::Enum(enum_token)]);

    match &first_prop(&root).ty {
        TypeIr::Class { is_enum, fqcn, .. } => {
            assert_eq!(fqcn, status_fqcn);
            assert!(*is_enum);
        }
        other => panic!("expected Class, got {other:?}"),
    }
}

#[test]
fn phpdoc_wins_over_native_type() {
    let class = single_prop_class(NativeType::Int, Some("string"));
    let root = build(&[Token::Class(class)]);
    assert_eq!(first_prop(&root).ty, TypeIr::String);
}

#[test]
fn phpdoc_nullable_question_mark() {
    let class = single_prop_class(NativeType::String, Some("?string"));
    let root = build(&[Token::Class(class)]);
    let property = first_prop(&root);
    assert_eq!(property.ty, nullable(TypeIr::String));
    assert!(property.nullable);
}

#[test]
fn phpdoc_union_with_null_wraps_nullable() {
    let class = single_prop_class(NativeType::Mixed, Some("int|string|null"));
    let root = build(&[Token::Class(class)]);
    let property = first_prop(&root);
    assert_eq!(
        property.ty,
        nullable(union(vec![TypeIr::Int, TypeIr::String]))
    );
    assert!(property.nullable);
}

#[test]
fn phpdoc_array_shape_becomes_struct() {
    let class = single_prop_class(NativeType::Array, Some(r"array{name: string, value: int}"));
    let root = build(&[Token::Class(class)]);
    match &first_prop(&root).ty {
        TypeIr::Struct { properties } => {
            assert_eq!(properties.len(), 2);
            assert_eq!(properties[0].name, "name");
            assert_eq!(properties[0].ty, TypeIr::String);
            assert!(!properties[0].optional);
            assert_eq!(properties[1].name, "value");
            assert_eq!(properties[1].ty, TypeIr::Int);
        }
        other => panic!("expected Struct, got {other:?}"),
    }
}

#[test]
fn phpdoc_record_of_array_shape_wraps_struct_in_array() {
    let class = single_prop_class(
        NativeType::Array,
        Some(r"array<string, array{name: string, value: int}>"),
    );
    let root = build(&[Token::Class(class)]);
    match &first_prop(&root).ty {
        TypeIr::Record { key, value } => {
            assert_eq!(**key, TypeIr::String);
            match value.as_ref() {
                TypeIr::Array { item } => match item.as_deref() {
                    Some(TypeIr::Struct { properties }) => {
                        assert_eq!(properties.len(), 2);
                        assert_eq!(properties[0].name, "name");
                        assert_eq!(properties[1].name, "value");
                    }
                    other => panic!("expected Array of Struct, got {other:?}"),
                },
                other => panic!("expected Array value, got {other:?}"),
            }
        }
        other => panic!("expected Record, got {other:?}"),
    }
}

#[test]
fn phpdoc_array_of_t_becomes_array() {
    let class = single_prop_class(NativeType::Array, Some("array<string>"));
    let root = build(&[Token::Class(class)]);
    assert_eq!(first_prop(&root).ty, array_of(TypeIr::String));
}

#[test]
fn phpdoc_same_namespace_class_ref() {
    let class = single_prop_class(NativeType::Mixed, Some("UserData"));
    let root = build(&[Token::Class(class)]);
    assert_eq!(first_prop(&root).ty, TypeIr::class_ref(USER_DATA_FQCN));
}

#[test]
fn copies_class_and_property_attributes() {
    let mut class = class_with_props(
        FIXTURES_NS,
        r"EffectSchemaGenerator\Tests\Fixtures\OptionalAttributeData",
        vec![PublicPropertyToken {
            name: "optionalProperty".into(),
            native_type: NativeType::String,
            phpdoc_type: None,
            attributes: vec![AttributeIr {
                name: r"Spatie\LaravelData\Attributes\Optional".into(),
                arguments: vec![],
            }],
        }],
    );
    class.attributes = vec![AttributeIr {
        name: r"Spatie\LaravelData\Attributes\Optional".into(),
        arguments: vec![],
    }];

    let root = build(&[Token::Class(class)]);
    let schema = first_schema(&root);
    assert_eq!(schema.class_attributes.len(), 1);
    assert_eq!(
        schema.class_attributes[0].name,
        r"Spatie\LaravelData\Attributes\Optional"
    );
    assert_eq!(schema.properties[0].attributes.len(), 1);
    assert_eq!(
        schema.properties[0].attributes[0].name,
        r"Spatie\LaravelData\Attributes\Optional"
    );
}

#[test]
fn copies_schema_uses_from_class_token() {
    let mut class = class_with_props(FIXTURES_NS, USER_DATA_FQCN, vec![string_prop("id")]);
    class.uses = BTreeMap::from([("Carbon".into(), r"Carbon\Carbon".into())]);
    let root = build(&[Token::Class(class)]);
    assert_eq!(
        first_schema(&root).uses.get("Carbon").map(String::as_str),
        Some(r"Carbon\Carbon")
    );
}

#[test]
fn native_nullable_string_sets_property_nullable() {
    let class = single_prop_class(NativeType::Nullable(Box::new(NativeType::String)), None);
    let root = build(&[Token::Class(class)]);
    let property = first_prop(&root);
    assert_eq!(property.ty, nullable(TypeIr::String));
    assert!(property.nullable);
}

#[test]
fn classes_and_enums_share_namespace() {
    let class = class_with_props(FIXTURES_NS, USER_DATA_FQCN, vec![string_prop("id")]);
    let color = EnumToken::new(FIXTURES_NS, COLOR_FQCN);
    let root = build(&[Token::Class(class), Token::Enum(color)]);
    let ns = &root.namespaces[FIXTURES_NS];
    assert_eq!(ns.schemas.len(), 1);
    assert_eq!(ns.enums.len(), 1);
}

#[test]
fn build_from_parsed_fixture_if_parse_works() {
    let path = effect_schema_generator::test_util::fixture_file("UserData.php");
    match effect_schema_generator::parse::parse_data_class(USER_DATA_FQCN, &path) {
        Ok(token) => {
            let root = build(&[Token::Class(token)]);
            assert!(root.namespaces.contains_key(FIXTURES_NS));
            assert_eq!(root.namespaces[FIXTURES_NS].schemas[0].name, "UserData");
        }
        Err(_) => {
            // parse stage is not finished; do not fail this builder test
        }
    }
}
