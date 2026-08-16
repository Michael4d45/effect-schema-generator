use std::path::PathBuf;

pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

pub fn fixture_file(rel: impl AsRef<std::path::Path>) -> PathBuf {
    fixtures_dir().join(rel)
}

pub fn app_fixtures_dir() -> PathBuf {
    fixtures_dir().join("src/app")
}
