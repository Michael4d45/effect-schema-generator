use effect_schema_generator::ir::TypeIr;
use effect_schema_generator::test_util::{app_fixtures_dir, fixtures_dir};
use effect_schema_generator::{generate, transform, Config};
use std::fs;
use std::path::Path;
use std::process::Command;

fn fixture_config(output: &Path) -> Config {
    Config {
        data_paths: vec![fixtures_dir()],
        enum_paths: vec![fixtures_dir()],
        output_directory: output.to_path_buf(),
        clear_output_directory_before_write: true,
        ..Config::default()
    }
}

fn app_config(output: &Path) -> Config {
    let app = app_fixtures_dir();
    Config {
        data_paths: vec![app.clone()],
        enum_paths: vec![app],
        output_directory: output.to_path_buf(),
        clear_output_directory_before_write: true,
        ..Config::default()
    }
}

fn schema_names(ir: &effect_schema_generator::RootIr, namespace: &str) -> Vec<String> {
    ir.namespaces
        .get(namespace)
        .map(|ns| ns.schemas.iter().map(|s| s.name.clone()).collect())
        .unwrap_or_default()
}

fn enum_names(ir: &effect_schema_generator::RootIr, namespace: &str) -> Vec<String> {
    ir.namespaces
        .get(namespace)
        .map(|ns| ns.enums.iter().map(|e| e.name.clone()).collect())
        .unwrap_or_default()
}

#[test]
fn generate_fixture_user_data_and_color() {
    let tmp = tempfile::tempdir().unwrap();
    let ir = generate(&fixture_config(tmp.path())).expect("generate fixtures");

    let ns = "EffectSchemaGenerator\\Tests\\Fixtures";
    let schemas = schema_names(&ir, ns);
    assert!(
        schemas.contains(&"UserData".into()),
        "missing UserData in {schemas:?}"
    );
    assert!(
        schemas.contains(&"AddressData".into()),
        "missing AddressData in {schemas:?}"
    );

    let enums = enum_names(&ir, ns);
    assert!(
        enums.contains(&"Color".into()),
        "missing Color in {enums:?}"
    );
    assert!(
        enums.contains(&"Priority".into()),
        "missing Priority in {enums:?}"
    );

    let user = ir
        .namespaces
        .get(ns)
        .unwrap()
        .schemas
        .iter()
        .find(|s| s.name == "UserData")
        .unwrap();
    let prop_names: Vec<_> = user.properties.iter().map(|p| p.name.as_str()).collect();
    for expected in ["id", "name", "email", "profile", "createdAt"] {
        assert!(
            prop_names.contains(&expected),
            "UserData missing {expected}: {prop_names:?}"
        );
    }

    let created = user
        .properties
        .iter()
        .find(|p| p.name == "createdAt")
        .unwrap();
    match &created.ty {
        TypeIr::Class { fqcn, .. } => assert_eq!(fqcn, "Carbon\\Carbon"),
        other => panic!("createdAt should be Carbon, got {other:?}"),
    }

    let profile = user
        .properties
        .iter()
        .find(|p| p.name == "profile")
        .unwrap();
    assert!(
        profile.nullable
            || matches!(profile.ty, TypeIr::Nullable { .. })
            || profile.attributes.iter().any(|a| a.name.contains("Lazy")),
        "profile should be nullable and/or Lazy: {profile:?}"
    );
}

#[test]
fn transform_writes_user_data_and_enum_files() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = fixture_config(tmp.path());
    transform(&cfg).expect("transform fixtures");

    let user = tmp
        .path()
        .join("EffectSchemaGenerator/Tests/Fixtures/UserData.ts");
    assert!(user.exists(), "missing {}", user.display());
    let content = fs::read_to_string(&user).unwrap();
    assert!(content.contains("export interface UserData"));
    assert!(content.contains("export interface UserDataEncoded"));
    assert!(content.contains("S.Struct"));
    assert!(content.contains("readonly id: string"));
    assert!(content.contains("readonly name: string"));
    assert!(content.contains("readonly email: string"));
    assert!(
        content.contains("Date") || content.contains("S.DateFromString"),
        "Date plugin should apply to createdAt: {content}"
    );
    assert!(
        content.contains("profile: ProfileData | null"),
        "profile is #[Lazy] ?ProfileData — PHP emits nullable, not optional: {content}"
    );
    assert!(content.ends_with('\n'));

    let color = tmp
        .path()
        .join("EffectSchemaGenerator/Tests/Fixtures/Color.ts");
    assert!(color.exists(), "missing {}", color.display());
    let color_ts = fs::read_to_string(&color).unwrap();
    assert!(
        color_ts.contains("export type Color") || color_ts.contains("export enum Color"),
        "enum writer missing Color type: {color_ts}"
    );

    let pagination = tmp.path().join("Illuminate/Pagination.ts");
    assert!(
        pagination.exists(),
        "paginator plugin should emit {}",
        pagination.display()
    );
}

#[test]
fn generate_app_fixtures_counts() {
    let tmp = tempfile::tempdir().unwrap();
    let ir = generate(&app_config(tmp.path())).expect("generate app fixtures");

    let events = schema_names(&ir, "App\\Data\\Events");
    let models = schema_names(&ir, "App\\Data\\Models");
    let requests = schema_names(&ir, "App\\Data\\Requests");
    let response = schema_names(&ir, "App\\Data\\Response");
    let enums = enum_names(&ir, "App\\Enums");

    assert_eq!(events.len(), 3, "Events: {events:?}");
    assert_eq!(models.len(), 25, "Models: {models:?}");
    assert_eq!(requests.len(), 13, "Requests: {requests:?}");
    assert_eq!(response.len(), 29, "Response: {response:?}");
    assert_eq!(enums.len(), 7, "Enums: {enums:?}");

    let features_recipe = schema_names(&ir, "App\\Features\\Recipe\\Responses");
    let features_activity = schema_names(&ir, "App\\Features\\Activity\\Responses");
    assert!(features_recipe.contains(&"ListRecipesResponse".into()));
    assert!(features_activity.contains(&"ActivitiesList".into()));
}

#[test]
fn transform_app_fixtures_writes_expected_files() {
    let tmp = tempfile::tempdir().unwrap();
    transform(&app_config(tmp.path())).expect("transform app fixtures");

    let user = tmp.path().join("App/Data/Models/UserData.ts");
    assert!(user.exists(), "missing {}", user.display());
    let content = fs::read_to_string(&user).unwrap();
    assert!(content.contains("export interface UserData"));
    assert!(content.contains("S.Struct"));

    let role = tmp.path().join("App/Enums/Role.ts");
    assert!(role.exists(), "missing {}", role.display());

    let ts_files = collect_ts_files(tmp.path());
    // 72 schemas + 7 enums + paginator helper (and maybe uploaded_file if referenced)
    assert!(
        ts_files.len() >= 79,
        "expected at least 72 schemas + 7 enums + helper, got {}: {ts_files:?}",
        ts_files.len()
    );
}

fn collect_ts_files(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(dir: &Path, out: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "ts") {
                out.push(path.display().to_string());
            }
        }
    }
    walk(dir, &mut out);
    out
}

#[test]
fn cli_transform_dry_run_lists_namespaces() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = fixture_config(tmp.path());
    let json = serde_json::to_string(&cfg).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_effect-schema"))
        .args(["transform", "--config-json", &json, "--dry-run"])
        .output()
        .expect("run effect-schema");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "cli failed: stdout={stdout} stderr={stderr}"
    );
    assert!(
        stdout.contains("Would generate"),
        "unexpected dry-run output: {stdout}"
    );
    assert!(
        stdout.contains("UserData") || stdout.contains("schema UserData"),
        "dry-run should list UserData: {stdout}"
    );
    assert!(
        !tmp.path()
            .join("EffectSchemaGenerator/Tests/Fixtures/UserData.ts")
            .exists(),
        "dry-run must not write files"
    );
}

#[test]
fn cli_transform_writes_files() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = fixture_config(tmp.path());
    let json = serde_json::to_string(&cfg).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_effect-schema"))
        .args(["transform", "--config-json", &json])
        .output()
        .expect("run effect-schema");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "cli failed: stdout={stdout} stderr={stderr}"
    );
    assert!(stdout.contains("Schema generation completed successfully"));
    assert!(tmp
        .path()
        .join("EffectSchemaGenerator/Tests/Fixtures/UserData.ts")
        .exists());
}
