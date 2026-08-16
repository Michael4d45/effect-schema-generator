use effect_schema_generator::config::Config;
use effect_schema_generator::ir::{
    AttributeIr, EnumCaseIr, EnumIr, NamespaceIr, PropertyIr, RootIr, SchemaIr, TypeIr,
};
use effect_schema_generator::plugins::load_plugins;
use effect_schema_generator::writer::{
    enum_file_path, relative_import_path, schema_file_path, write,
};
use std::fs;
use std::path::Path;

#[test]
fn relative_import_same_directory() {
    assert_eq!(
        relative_import_path("App/Data/UserData.ts", "App/Data/Foo.ts"),
        "./Foo"
    );
}

#[test]
fn relative_import_parent_directory() {
    assert_eq!(
        relative_import_path("App/Data/UserData.ts", "App/Enums/Color.ts"),
        "../Enums/Color"
    );
}

#[test]
fn relative_import_nested_down() {
    assert_eq!(
        relative_import_path("App/User.ts", "App/Data/Nested/Foo.ts"),
        "./Data/Nested/Foo"
    );
}

#[test]
fn relative_import_nested_up() {
    assert_eq!(
        relative_import_path("App/Data/Nested/User.ts", "App/Foo.ts"),
        "../../Foo"
    );
}

#[test]
fn schema_and_enum_file_paths_use_namespace() {
    assert_eq!(
        schema_file_path("App\\Data", "UserData"),
        "App/Data/UserData.ts"
    );
    assert_eq!(enum_file_path("App\\Enums", "Color"), "App/Enums/Color.ts");
}

fn write_cfg(dir: &Path, plugins: &[&str]) -> Config {
    Config {
        output_directory: dir.to_path_buf(),
        clear_output_directory_before_write: false,
        plugins: plugins.iter().map(|s| (*s).to_string()).collect(),
        ..Config::default()
    }
}

fn user_data_ir() -> RootIr {
    let mut ns = NamespaceIr::new("App\\Data");
    ns.schemas.push(SchemaIr::new(
        "UserData",
        vec![
            PropertyIr::new("name", TypeIr::String),
            PropertyIr::new("createdAt", TypeIr::class_ref("Carbon\\Carbon")),
            PropertyIr::new(
                "tags",
                TypeIr::class_ref_params(
                    "Illuminate\\Support\\Collection",
                    vec![TypeIr::String, TypeIr::String],
                ),
            ),
        ],
    ));
    let mut root = RootIr::default();
    root.namespaces.insert(ns.namespace.clone(), ns);
    root
}

#[test]
fn write_schema_file_applies_date_and_collection() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = write_cfg(tmp.path(), &["date", "collection"]);
    let plugins = load_plugins(&cfg);
    write(&user_data_ir(), &plugins, &cfg).unwrap();

    let path = tmp.path().join("App/Data/UserData.ts");
    assert!(path.exists(), "expected {}", path.display());
    let content = fs::read_to_string(&path).unwrap();

    assert!(content.contains("export interface UserData"));
    assert!(content.contains("name"));
    assert!(content.contains("createdAt"));
    assert!(content.contains("tags"));
    assert!(content.contains("Date"), "Date plugin should apply");
    assert!(
        content.contains("readonly string[]")
            || content.contains("readonly tags: readonly string[]"),
        "Collection should become an array: {content}"
    );
    assert!(content.contains("S.Struct"));
    assert!(content.contains("export interface UserDataEncoded"));
    assert!(content.contains("S.DateFromString"));
    assert!(content.ends_with('\n'));
    assert!(!content.ends_with("\n\n"));
}

#[test]
fn write_omits_hidden_properties() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = write_cfg(tmp.path(), &["hidden"]);
    let plugins = load_plugins(&cfg);

    let mut ns = NamespaceIr::new("App\\Data");
    ns.schemas.push(SchemaIr::new(
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
    ));
    let mut root = RootIr::default();
    root.namespaces.insert(ns.namespace.clone(), ns);

    write(&root, &plugins, &cfg).unwrap();
    let content = fs::read_to_string(tmp.path().join("App/Data/UserData.ts")).unwrap();
    assert!(content.contains("name"));
    assert!(
        !content.contains("secret"),
        "hidden property should be omitted: {content}"
    );
}

#[test]
fn write_optional_property_has_question_mark() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = write_cfg(tmp.path(), &[]);
    let plugins = load_plugins(&cfg);

    let mut ns = NamespaceIr::new("App\\Data");
    ns.schemas.push(SchemaIr::new(
        "UserData",
        vec![PropertyIr {
            optional: true,
            ..PropertyIr::new("name", TypeIr::String)
        }],
    ));
    let mut root = RootIr::default();
    root.namespaces.insert(ns.namespace.clone(), ns);

    write(&root, &plugins, &cfg).unwrap();
    let content = fs::read_to_string(tmp.path().join("App/Data/UserData.ts")).unwrap();
    assert!(
        content.contains("name?:") || content.contains("readonly name?:"),
        "optional property should use ?: {content}"
    );
}

#[test]
fn write_generates_enum_file() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = write_cfg(
        tmp.path(),
        &["type_enum", "effect_schema_enum", "const_object_enum"],
    );
    let plugins = load_plugins(&cfg);

    let mut ns = NamespaceIr::new("App\\Enums");
    ns.enums.push(EnumIr::new(
        "Color",
        vec![
            EnumCaseIr::new("RED", "red"),
            EnumCaseIr::new("BLUE", "blue"),
        ],
        "string",
    ));
    let mut root = RootIr::default();
    root.namespaces.insert(ns.namespace.clone(), ns);

    write(&root, &plugins, &cfg).unwrap();
    let path = tmp.path().join("App/Enums/Color.ts");
    assert!(path.exists(), "expected {}", path.display());
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("export type Color = \"red\" | \"blue\";"));
    assert!(content.contains("S.Literal(\"red\")"));
    assert!(content.contains("RED: 'red'"));
    assert!(content.ends_with('\n'));
}

#[test]
fn write_transformer_provided_files() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = write_cfg(tmp.path(), &["length_aware_paginator"]);
    let plugins = load_plugins(&cfg);
    write(&RootIr::default(), &plugins, &cfg).unwrap();
    let path = tmp.path().join("Illuminate/Pagination.ts");
    assert!(path.exists(), "expected {}", path.display());
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("export const LengthAwarePaginatorSchema"));
}

#[test]
fn write_clears_output_directory_contents() {
    let tmp = tempfile::tempdir().unwrap();
    let junk = tmp.path().join("junk.txt");
    fs::write(&junk, "stale").unwrap();
    let nested = tmp.path().join("old");
    fs::create_dir(&nested).unwrap();
    fs::write(nested.join("x.ts"), "stale").unwrap();

    let mut cfg = write_cfg(tmp.path(), &["date"]);
    cfg.clear_output_directory_before_write = true;
    let plugins = load_plugins(&cfg);
    write(&user_data_ir(), &plugins, &cfg).unwrap();

    assert!(!junk.exists());
    assert!(!nested.exists());
    assert!(tmp.path().is_dir());
    assert!(tmp.path().join("App/Data/UserData.ts").exists());
}

#[test]
fn write_nullable_and_imports() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = write_cfg(
        tmp.path(),
        &["type_enum", "effect_schema_enum", "const_object_enum"],
    );
    let plugins = load_plugins(&cfg);

    let mut enum_ns = NamespaceIr::new("App\\Enums");
    enum_ns.enums.push(EnumIr::new(
        "Color",
        vec![EnumCaseIr::new("RED", "red")],
        "string",
    ));

    let mut data_ns = NamespaceIr::new("App\\Data");
    data_ns.schemas.push(SchemaIr::new(
        "UserData",
        vec![
            PropertyIr {
                nullable: true,
                ..PropertyIr::new("id", TypeIr::String)
            },
            PropertyIr::new("favoriteColor", TypeIr::enum_ref("App\\Enums\\Color")),
        ],
    ));

    let mut root = RootIr::default();
    root.namespaces.insert(enum_ns.namespace.clone(), enum_ns);
    root.namespaces.insert(data_ns.namespace.clone(), data_ns);

    write(&root, &plugins, &cfg).unwrap();
    let content = fs::read_to_string(tmp.path().join("App/Data/UserData.ts")).unwrap();
    assert!(content.contains("string | null"));
    assert!(content.contains("from '../Enums/Color'"));
    assert!(content.contains("favoriteColor: Color"));
}
