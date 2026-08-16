use effect_schema_generator::discovery::{discover_data_classes, discover_enums, scan_php_files};
use effect_schema_generator::test_util::{app_fixtures_dir, fixtures_dir};
use effect_schema_generator::Config;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::PathBuf;

fn config_with_paths(data_paths: Vec<PathBuf>, enum_paths: Vec<PathBuf>) -> Config {
    Config {
        data_paths,
        enum_paths,
        ..Config::default()
    }
}

fn unique_sorted(items: &[String]) -> Vec<String> {
    let mut out = items.to_vec();
    out.sort();
    out.dedup();
    out
}

#[test]
fn discovers_fixture_data_classes() {
    let config = config_with_paths(vec![fixtures_dir()], vec![]);
    let classes = discover_data_classes(&config).expect("discover data classes");

    assert!(
        !classes.is_empty(),
        "expected fixture Data classes under fixtures/"
    );
    assert!(
        classes.contains(&"EffectSchemaGenerator\\Tests\\Fixtures\\UserData".to_string()),
        "missing UserData: {classes:?}"
    );
    assert!(
        classes.contains(&"EffectSchemaGenerator\\Tests\\Fixtures\\AddressData".to_string()),
        "missing AddressData: {classes:?}"
    );
    // Parent in the same scan set extends Spatie Data.
    assert!(
        classes.contains(&"EffectSchemaGenerator\\Tests\\Fixtures\\IndexRoutesRequest".to_string()),
        "IndexRoutesRequest extends PaginationRequest which extends Data: {classes:?}"
    );
    assert!(
        !classes.iter().any(|c| c.ends_with("\\HasPagination")
            || c.ends_with("\\HasVisibility")
            || c.ends_with("\\OptionalTrait")),
        "traits must not be treated as Data classes: {classes:?}"
    );
    assert_eq!(
        classes,
        unique_sorted(&classes),
        "FQCNs must be unique and sorted"
    );
}

#[test]
fn discovers_fixture_enums() {
    let config = config_with_paths(vec![], vec![fixtures_dir()]);
    let enums = discover_enums(&config).expect("discover enums");

    assert!(!enums.is_empty(), "expected fixture enums under fixtures/");
    assert!(
        enums.contains(&"EffectSchemaGenerator\\Tests\\Fixtures\\Color".to_string()),
        "missing Color: {enums:?}"
    );
    assert!(
        enums.contains(&"EffectSchemaGenerator\\Tests\\Fixtures\\Priority".to_string()),
        "missing Priority: {enums:?}"
    );
    assert!(
        enums.contains(&"EffectSchemaGenerator\\Tests\\Fixtures\\TestStatus".to_string()),
        "missing TestStatus: {enums:?}"
    );
    assert_eq!(
        enums,
        unique_sorted(&enums),
        "FQCNs must be unique and sorted"
    );
}

#[test]
fn scan_php_files_returns_path_fqcn_pairs() {
    let scanned = scan_php_files(&[fixtures_dir()]).expect("scan php files");
    let fqcns: Vec<_> = scanned.iter().map(|(_, fqcn)| fqcn.as_str()).collect();

    assert!(
        fqcns.contains(&"EffectSchemaGenerator\\Tests\\Fixtures\\UserData"),
        "scan should extract UserData FQCN"
    );
    assert!(
        fqcns.contains(&"EffectSchemaGenerator\\Tests\\Fixtures\\Color"),
        "scan should extract Color enum FQCN"
    );
    assert!(scanned
        .iter()
        .any(|(path, fqcn)| path.extension().is_some_and(|e| e == "php")
            && fqcn == "EffectSchemaGenerator\\Tests\\Fixtures\\UserData"));
}

#[test]
fn missing_paths_yield_empty_lists() {
    let missing = PathBuf::from("/non/existent/path");
    let config = config_with_paths(vec![missing.clone()], vec![missing.clone()]);

    let classes = discover_data_classes(&config).expect("missing data path");
    let enums = discover_enums(&config).expect("missing enum path");
    let scanned = scan_php_files(&[missing]).expect("missing scan path");

    assert!(classes.is_empty());
    assert!(enums.is_empty());
    assert!(scanned.is_empty());
}

#[test]
fn empty_paths_yield_empty_lists() {
    let config = config_with_paths(vec![], vec![]);

    let classes = discover_data_classes(&config).expect("empty data paths");
    let enums = discover_enums(&config).expect("empty enum paths");
    let scanned = scan_php_files(&[]).expect("empty scan paths");

    assert!(classes.is_empty());
    assert!(enums.is_empty());
    assert!(scanned.is_empty());
}

#[test]
fn skips_files_without_namespace_or_class() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("empty.php"), "").unwrap();
    fs::write(dir.path().join("echo.php"), "<?php echo \"test\";").unwrap();
    fs::write(dir.path().join("no_namespace.php"), "<?php class Orphan {}").unwrap();

    let config = config_with_paths(
        vec![dir.path().to_path_buf()],
        vec![dir.path().to_path_buf()],
    );
    let classes = discover_data_classes(&config).expect("invalid files data");
    let enums = discover_enums(&config).expect("invalid files enums");
    let scanned = scan_php_files(&[dir.path().to_path_buf()]).expect("invalid files scan");

    assert!(classes.is_empty(), "unexpected classes: {classes:?}");
    assert!(enums.is_empty(), "unexpected enums: {enums:?}");
    assert!(scanned.is_empty(), "unexpected scan results: {scanned:?}");
}

#[test]
fn skips_unparseable_files_without_crashing() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("broken.php"), b"<?php class {{{").unwrap();
    fs::write(
        dir.path().join("GoodData.php"),
        r#"<?php
namespace Test\Namespace;
use Spatie\LaravelData\Data;
class GoodData extends Data {}
"#,
    )
    .unwrap();

    let config = config_with_paths(vec![dir.path().to_path_buf()], vec![]);
    let classes = discover_data_classes(&config).expect("parse failure should be skipped");

    assert_eq!(classes, vec!["Test\\Namespace\\GoodData".to_string()]);
}

#[test]
fn skips_blade_php_files() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("Hidden.blade.php"),
        r#"<?php
namespace Test\Namespace;
use Spatie\LaravelData\Data;
class Hidden extends Data {}
"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("Visible.php"),
        r#"<?php
namespace Test\Namespace;
use Spatie\LaravelData\Data;
class Visible extends Data {}
"#,
    )
    .unwrap();

    let classes = discover_data_classes(&config_with_paths(vec![dir.path().to_path_buf()], vec![]))
        .expect("blade skip");
    assert_eq!(classes, vec!["Test\\Namespace\\Visible".to_string()]);
}

#[test]
fn discovers_ast_extends_without_executing_php() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("BadDataClass.php"),
        r#"<?php
namespace Test\Namespace;
class BadDataClass extends \Spatie\LaravelData\Data {
}
"#,
    )
    .unwrap();

    let classes = discover_data_classes(&config_with_paths(vec![dir.path().to_path_buf()], vec![]))
        .expect("direct FQCN extends");
    assert_eq!(classes, vec!["Test\\Namespace\\BadDataClass".to_string()]);
}

#[test]
fn walks_parent_chain_within_scan_set() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("Base.php"),
        r#"<?php
namespace Test\Chain;
use Spatie\LaravelData\Data;
class Base extends Data {}
"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("Child.php"),
        r#"<?php
namespace Test\Chain;
class Child extends Base {}
"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("Unrelated.php"),
        r#"<?php
namespace Test\Chain;
class Unrelated extends \stdClass {}
"#,
    )
    .unwrap();

    let classes = discover_data_classes(&config_with_paths(vec![dir.path().to_path_buf()], vec![]))
        .expect("parent chain");
    assert_eq!(
        classes,
        vec![
            "Test\\Chain\\Base".to_string(),
            "Test\\Chain\\Child".to_string(),
        ]
    );
}

#[test]
fn discovers_app_fixture_data_classes_and_enums() {
    let app = app_fixtures_dir();
    let config = config_with_paths(vec![app.clone()], vec![app.clone()]);

    let classes = discover_data_classes(&config).expect("app data classes");
    let enums = discover_enums(&config).expect("app enums");

    let app_data: Vec<_> = classes
        .iter()
        .filter(|c| c.starts_with("App\\Data\\"))
        .cloned()
        .collect();
    let app_features: Vec<_> = classes
        .iter()
        .filter(|c| c.starts_with("App\\Features\\"))
        .cloned()
        .collect();
    let app_enums: Vec<_> = enums
        .iter()
        .filter(|c| c.starts_with("App\\Enums\\"))
        .cloned()
        .collect();

    // 70 Data classes: 3 Events + 25 Models + 13 Requests + 29 Response
    assert_eq!(app_data.len(), 70, "App\\Data classes: {app_data:?}");
    assert_eq!(
        app_data
            .iter()
            .filter(|c| c.starts_with("App\\Data\\Events\\"))
            .count(),
        3
    );
    assert_eq!(
        app_data
            .iter()
            .filter(|c| c.starts_with("App\\Data\\Models\\"))
            .count(),
        25
    );
    assert_eq!(
        app_data
            .iter()
            .filter(|c| c.starts_with("App\\Data\\Requests\\"))
            .count(),
        13
    );
    assert_eq!(
        app_data
            .iter()
            .filter(|c| c.starts_with("App\\Data\\Response\\"))
            .count(),
        29
    );

    assert_eq!(
        app_features.len(),
        2,
        "expected Features Data classes: {app_features:?}"
    );
    assert!(
        app_features.contains(&"App\\Features\\Recipe\\Responses\\ListRecipesResponse".to_string())
    );
    assert!(
        app_features.contains(&"App\\Features\\Activity\\Responses\\ActivitiesList".to_string())
    );

    // 7 Enums; EnumUtil.php is a trait, not an enum
    assert_eq!(app_enums.len(), 7, "App\\Enums: {app_enums:?}");
    for name in [
        "App\\Enums\\CredentialType",
        "App\\Enums\\EventType",
        "App\\Enums\\QuestionType",
        "App\\Enums\\Role",
        "App\\Enums\\SessionStatus",
        "App\\Enums\\SortDirection",
        "App\\Enums\\Visibility",
    ] {
        assert!(
            app_enums.iter().any(|e| e == name),
            "missing {name}: {app_enums:?}"
        );
    }
    assert!(
        !app_enums.iter().any(|e| e.ends_with("\\EnumUtil")),
        "EnumUtil trait must not be discovered as an enum"
    );

    assert_eq!(classes, unique_sorted(&classes));
    assert_eq!(enums, unique_sorted(&enums));
    assert_eq!(classes.len(), 72, "70 App\\Data + 2 Features");
}
