use std::collections::BTreeMap;

use effect_schema_generator::ir::EnumCaseIr;
use effect_schema_generator::parse::{
    parse_data_class, parse_enum, parse_use_statements, phpdoc_type_string,
};
use effect_schema_generator::test_util::fixture_file;
use effect_schema_generator::tokens::{ClassToken, NativeType, PublicPropertyToken};
use pretty_assertions::assert_eq;

const FIXTURES_NS: &str = "EffectSchemaGenerator\\Tests\\Fixtures";

fn fqcn(name: &str) -> String {
    format!("{FIXTURES_NS}\\{name}")
}

fn parse_fixture_class(name: &str) -> ClassToken {
    let file = fixture_file(format!("{name}.php"));
    parse_data_class(&fqcn(name), &file)
        .unwrap_or_else(|err| panic!("parse_data_class({name}) failed: {err}"))
}

fn prop<'a>(class: &'a ClassToken, name: &str) -> &'a PublicPropertyToken {
    class
        .public_properties
        .iter()
        .find(|property| property.name == name)
        .unwrap_or_else(|| {
            let names: Vec<_> = class
                .public_properties
                .iter()
                .map(|property| property.name.as_str())
                .collect();
            panic!("missing property {name}; have {names:?}")
        })
}

fn prop_names(class: &ClassToken) -> Vec<&str> {
    class
        .public_properties
        .iter()
        .map(|property| property.name.as_str())
        .collect()
}

fn attr_names(property: &PublicPropertyToken) -> Vec<&str> {
    property
        .attributes
        .iter()
        .map(|attribute| attribute.name.as_str())
        .collect()
}

#[test]
fn parse_use_statements_simple_alias_group_and_skips() {
    let uses = parse_use_statements(
        r#"<?php

namespace App\Data;

use App\Models\User;
use Carbon\Carbon as Date;
use Illuminate\Support\{Collection, Enumerable as Enumerables};
use function strlen;
use const PHP_EOL;

$fn = function () use ($x) {
    return $x;
};

class Example
{
    use SomeTrait;
}
"#,
    )
    .expect("parse_use_statements");

    let expected: BTreeMap<String, String> = [
        ("User", "App\\Models\\User"),
        ("Date", "Carbon\\Carbon"),
        ("Collection", "Illuminate\\Support\\Collection"),
        ("Enumerables", "Illuminate\\Support\\Enumerable"),
    ]
    .into_iter()
    .map(|(alias, fqcn)| (alias.to_string(), fqcn.to_string()))
    .collect();

    assert_eq!(uses, expected);
}

#[test]
fn parse_user_data() {
    let class = parse_fixture_class("UserData");

    assert_eq!(class.namespace, FIXTURES_NS);
    assert_eq!(class.fqcn, fqcn("UserData"));
    assert_eq!(
        prop_names(&class),
        vec!["id", "name", "email", "profile", "createdAt"]
    );

    assert_eq!(prop(&class, "id").native_type, NativeType::String);
    assert_eq!(prop(&class, "name").native_type, NativeType::String);
    assert_eq!(prop(&class, "email").native_type, NativeType::String);
    assert_eq!(
        prop(&class, "profile").native_type,
        NativeType::Nullable(Box::new(NativeType::Class(fqcn("ProfileData"))))
    );
    assert_eq!(
        prop(&class, "createdAt").native_type,
        NativeType::Class("Carbon\\Carbon".to_string())
    );

    let profile_attrs = attr_names(prop(&class, "profile"));
    assert_eq!(profile_attrs, vec!["Spatie\\LaravelData\\Attributes\\Lazy"]);
    assert!(prop(&class, "profile").attributes[0].arguments.is_empty());

    assert_eq!(
        class.uses.get("Carbon").map(String::as_str),
        Some("Carbon\\Carbon")
    );
    assert_eq!(
        class.uses.get("Data").map(String::as_str),
        Some("Spatie\\LaravelData\\Data")
    );
    assert_eq!(
        class.uses.get("Lazy").map(String::as_str),
        Some("Spatie\\LaravelData\\Attributes\\Lazy")
    );
}

#[test]
fn parse_constructor_param_data_phpdoc() {
    let class = parse_fixture_class("ConstructorParamData");

    assert_eq!(prop_names(&class), vec!["items", "name"]);
    assert_eq!(prop(&class, "items").native_type, NativeType::Array);
    assert_eq!(prop(&class, "name").native_type, NativeType::String);
    assert_eq!(
        prop(&class, "items").phpdoc_type.as_deref(),
        Some("array<int, string>")
    );
    assert_eq!(prop(&class, "name").phpdoc_type.as_deref(), Some("string"));
}

#[test]
fn parse_complex_data_unions_nullable_and_phpdoc() {
    let class = parse_fixture_class("ComplexData");

    for name in [
        "identifier",
        "optionalIdentifier",
        "mixedValue",
        "users",
        "lazyUsers",
        "nestedStrings",
        "optionalProfile",
        "firstName",
        "createdAt",
        "updatedAt",
        "address",
        "tags",
        "metadata",
        "relatedItems",
        "settings",
    ] {
        let _ = prop(&class, name);
    }

    assert_eq!(
        prop(&class, "identifier").native_type,
        NativeType::Union(vec![NativeType::Int, NativeType::String])
    );
    assert_eq!(
        prop(&class, "optionalIdentifier").native_type,
        NativeType::Nullable(Box::new(NativeType::Union(vec![
            NativeType::Int,
            NativeType::String,
        ])))
    );
    assert_eq!(
        prop(&class, "mixedValue").native_type,
        NativeType::Nullable(Box::new(NativeType::Union(vec![
            NativeType::Int,
            NativeType::Float,
            NativeType::String,
            NativeType::Bool,
        ])))
    );
    assert_eq!(
        prop(&class, "users").native_type,
        NativeType::Class("Illuminate\\Support\\Collection".to_string())
    );
    assert_eq!(
        prop(&class, "users").phpdoc_type.as_deref(),
        Some("Collection<array-key, UserData>")
    );
    assert_eq!(prop(&class, "tags").native_type, NativeType::Array);

    let first_name_attrs = attr_names(prop(&class, "firstName"));
    assert!(first_name_attrs.contains(&"Spatie\\LaravelData\\Attributes\\MapInputName"));
    assert!(first_name_attrs.contains(&"Spatie\\LaravelData\\Attributes\\MapOutputName"));
    assert_eq!(
        prop(&class, "firstName").attributes[0].arguments,
        vec![serde_json::json!("first_name")]
    );

    assert!(attr_names(prop(&class, "optionalProfile"))
        .contains(&"Spatie\\LaravelData\\Attributes\\Lazy"));
}

#[test]
fn parse_class_with_optional_trait_inherits_properties_and_attributes() {
    let class = parse_fixture_class("ClassWithOptionalTrait");

    for name in [
        "traitOptionalProperty",
        "traitRequiredProperty",
        "classProperty",
    ] {
        let _ = prop(&class, name);
    }

    assert_eq!(
        attr_names(prop(&class, "traitOptionalProperty")),
        vec!["Spatie\\LaravelData\\Attributes\\Optional"]
    );
    assert!(prop(&class, "traitRequiredProperty").attributes.is_empty());
    assert!(prop(&class, "classProperty").attributes.is_empty());
    assert_eq!(
        class.uses.get("Optional").map(String::as_str),
        Some("Spatie\\LaravelData\\Attributes\\Optional")
    );
}

#[test]
fn parse_color_enum_and_priority() {
    let color = parse_enum(&fqcn("Color"), &fixture_file("Color.php")).expect("Color");
    assert_eq!(color.fqcn, fqcn("Color"));
    assert_eq!(color.namespace, FIXTURES_NS);
    assert_eq!(color.backed_type, "");
    assert_eq!(
        color
            .cases
            .iter()
            .map(|case| case.name.as_str())
            .collect::<Vec<_>>(),
        vec!["RED", "GREEN", "BLUE", "YELLOW", "BLACK", "WHITE"]
    );
    for case in &color.cases {
        assert_eq!(case.value, None, "pure enum case {}", case.name);
    }

    let priority = parse_enum(&fqcn("Priority"), &fixture_file("Priority.php")).expect("Priority");
    assert_eq!(priority.fqcn, fqcn("Priority"));
    assert_eq!(priority.backed_type, "int");
    assert_eq!(
        priority.cases,
        vec![
            EnumCaseIr {
                name: "LOW".into(),
                value: Some(serde_json::json!(1)),
            },
            EnumCaseIr {
                name: "MEDIUM".into(),
                value: Some(serde_json::json!(2)),
            },
            EnumCaseIr {
                name: "HIGH".into(),
                value: Some(serde_json::json!(3)),
            },
            EnumCaseIr {
                name: "URGENT".into(),
                value: Some(serde_json::json!(4)),
            },
        ]
    );

    let status = parse_enum(&fqcn("TestStatus"), &fixture_file("TestStatus.php"))
        .expect("TestStatus string-backed enum");
    assert_eq!(status.backed_type, "string");
    assert_eq!(
        status.cases[0],
        EnumCaseIr {
            name: "ACTIVE".into(),
            value: Some(serde_json::json!("active")),
        }
    );
}

#[test]
fn parse_address_data_apartment_is_nullable() {
    let class = parse_fixture_class("AddressData");

    for name in [
        "street",
        "city",
        "state",
        "zipCode",
        "country",
        "apartment",
        "latitude",
        "longitude",
        "isPrimary",
    ] {
        let _ = prop(&class, name);
    }

    assert_eq!(
        prop(&class, "apartment").native_type,
        NativeType::Nullable(Box::new(NativeType::String))
    );
    assert_eq!(prop(&class, "latitude").native_type, NativeType::Float);
    assert_eq!(prop(&class, "isPrimary").native_type, NativeType::Bool);
}

#[test]
fn parse_index_route_events_request_inherits_trait_properties() {
    let class = parse_fixture_class("IndexRouteEventsRequest");

    for name in [
        "per_page",
        "columns",
        "page_name",
        "page",
        "sort_directions",
        "sort_by",
        "visibility",
    ] {
        let _ = prop(&class, name);
    }

    assert_eq!(
        prop(&class, "per_page").native_type,
        NativeType::Nullable(Box::new(NativeType::Int))
    );
    assert_eq!(
        prop(&class, "visibility").native_type,
        NativeType::Nullable(Box::new(NativeType::Class(format!(
            "{FIXTURES_NS}\\Enums\\Visibility"
        ))))
    );
    assert_eq!(
        prop(&class, "columns").phpdoc_type.as_deref(),
        Some("null|list<string>")
    );
}

#[test]
fn phpdoc_type_string_helper_extracts_var_and_param() {
    assert_eq!(
        phpdoc_type_string("/** @var Collection<array-key, UserData> */", None).as_deref(),
        Some("Collection<array-key, UserData>")
    );
    assert_eq!(
        phpdoc_type_string("/** @param array<int, string> $items */", Some("items")).as_deref(),
        Some("array<int, string>")
    );
}

#[test]
fn mixed_is_mixed_not_nullable_class() {
    let class = parse_fixture_class("EdgeCaseData");
    assert_eq!(prop(&class, "mixedProperty").native_type, NativeType::Mixed);
}
