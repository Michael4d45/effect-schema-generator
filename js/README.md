# @michael4d45/effect-schema

Generate TypeScript interfaces and Effect schemas from Spatie Laravel Data classes. The CLI is a native Rust binary; this package is the Node shim, Vite plugin, and platform-specific optional binaries.

## Install

```bash
npm install -D @michael4d45/effect-schema
```

`npm install` ships native binaries under `binaries/` (linux gnu/musl, darwin). Alpine CI, Docker Vite, and Lerd on Linux use the musl or gnu Linux build; macOS Lerd uses the darwin build. Override with `EFFECT_SCHEMA_BIN`.

## CLI

```bash
effect-schema transform --config effect-schema.toml
```

## Vite

```ts
import { laravelDataTypes } from '@michael4d45/effect-schema/vite';

export default defineConfig({
  plugins: [laravelDataTypes()],
});
```

The plugin reads `data_paths` / `enum_paths` from `effect-schema.toml` and regenerates types on PHP HMR. It does not boot Laravel.

Override the binary with `EFFECT_SCHEMA_BIN` or `{ binary: '/path/to/effect-schema' }`.

## PHP attributes

Keep `#[Optional]`, `#[Hidden]`, and `#[SnakeCase]` on Data classes. Those live in the Composer package `michael4d45/effect-schema-generator` (attributes + optional Artisan wrapper). Generation itself should go through this npm package so Alpine CI and Vite never wait on Laravel boot.
