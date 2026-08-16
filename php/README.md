# michael4d45/effect-schema-generator

Laravel attributes (`Optional`, `Hidden`, `SnakeCase`) and an Artisan wrapper around the **npm** `effect-schema` binary.

Type generation should run from Node (`npx effect-schema transform` or the Vite plugin in `@michael4d45/effect-schema/vite`) so Alpine CI and Vite do not boot Laravel. This package exists so Data classes can keep using `EffectSchemaGenerator\Attributes\*` and so `php artisan effect-schema:transform` still works when `node_modules/@michael4d45/effect-schema-*` is installed.

```bash
composer require michael4d45/effect-schema-generator
npm install -D @michael4d45/effect-schema
```

The wrapper prefers `EFFECT_SCHEMA_BIN`, then the platform package under `node_modules/@michael4d45/`, then a local `cargo` build.
