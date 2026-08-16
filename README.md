# effect-schema-generator (Rust)

Native generator for TypeScript / Effect schemas from Spatie Laravel Data classes. PHP source of truth: [Laravel-Effect-Generator](https://github.com/Michael4d45/Laravel-Effect-Generator).

## Packages

| Distro | Path | Who consumes it |
|---|---|---|
| npm `@michael4d45/effect-schema` | `js/` | Vite, `npm run types:generate`, CI |
| Composer `michael4d45/effect-schema-generator` | `php/` | `#[Optional]` attributes + Artisan wrapper |
| Cargo `effect-schema-generator` | crate root | the `effect-schema` binary |

Apps should **generate types from npm**, not from `php artisan`, so Alpine Docker/CI and Lerd Vite do not boot Laravel (~0.9s) and do not need a sibling `cargo build`.

## Local binary

```bash
cargo test
cargo build --release
./target/release/effect-schema transform --config effect-schema.toml
```

Musl (Alpine / static Linux):

```bash
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl
# see scripts/build-npm-binaries.sh
```

## Publish

GitHub Actions (`.github/workflows/release.yml`) builds platform binaries, copies them into `js/binaries/`, and `npm publish`s `@michael4d45/effect-schema` via npm Trusted Publishing (OIDC). No `NPM_TOKEN` secret.

```bash
git tag v0.1.1
git push origin v0.1.1
```

Composer wrapper is published separately to Packagist from `php/` (attributes + Artisan; looks up the npm native binary under `node_modules/@michael4d45/effect-schema-*`).
