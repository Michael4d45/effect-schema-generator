# Pipeline outline

Rust port of `Laravel-Effect-Generator`. Same stages, mago crates instead of PHP reflection / phpstan phpdoc-parser.

PHP source of truth: `/home/michael/Projects/Laravel-Effect-Generator`

## Stages (PHP → Rust)

```
paths in config
        │
        ▼
[1] Discovery          PHP: Discovery/*          Rust: src/discovery
        │  FQCNs of Data classes + enums
        ▼
[2] Parse              PHP: Reflection/*         Rust: src/parse  (mago-syntax, mago-names, mago-docblock)
        │  ClassToken / EnumToken
        ▼
[3] Build IR           PHP: Builder/*            Rust: src/builder
        │  RootIR (namespaces → schemas/enums → properties → TypeIR)
        ▼
[4] Plugins            PHP: Plugins/*            Rust: src/plugins
        │  mutate IR + context-aware TS snippets
        ▼
[5] Write              PHP: Writer/*             Rust: src/writer
        │  .ts files (interface, encoded interface, Effect schema, enums)
        ▼
[6] CLI / Laravel wrap PHP: artisan command      Rust: src/bin + php/
```

### 1. Discovery

Scan configured directories for `*.php` (skip `*.blade.php`). Extract FQCN via namespace + `class`/`enum` name.

A class is a Spatie Data class if its AST `extends` includes `Spatie\LaravelData\Data` (walk parent chain in parsed files; do not execute PHP).

An enum is a native PHP `enum`.

PHP: `PhpClassCandidateScanner`, `SpatieDataClassDiscoverer`, `NativeEnumDiscoverer`, `ClassDiscoverer`.

### 2. Parse (mago)

For each PHP file, `mago_syntax::parser::parse_file` / `parse_file_content` on a `mago_database::file::File` in a `mago_allocator` arena.

Use `mago-names` to resolve imports / short names to FQCNs.

Use `mago-docblock` (`parse_phpdoc_with_span` / `parse_trivia`) for `@var` and constructor `@param`.

Produce:

- `ClassToken`: namespace, fqcn, uses (alias → FQCN), public properties (name, native type, phpdoc type, attributes), class attributes
- `EnumToken`: fqcn, namespace, backed type, cases

Include inherited public properties from parents and traits (parse those files too). Skip static properties.

PHP: `DataClassParser`, `EnumParser`, `UseStatementParser`.

### 3. Build IR

PHPDoc wins over native types when present (`phpdoc_overrides_types`, default true).

Map to `TypeIR`:

- primitives: string, int, float, bool
- nullable, union
- array / list → `Array`
- `array<K,V>` → `Record`
- array shapes `{name: string}` → `Struct`
- class/enum references (generics as type parameters)
- unknown / mixed → `Unknown`

Mark enum class-references using the discovered enum FQCN set.

PHP: `AstBuilder`, `PhpDocTypeBuilder`, `NativeTypeBuilder`.

### 4. Plugins (transformers)

Same default set as PHP `config/effect-schema.php`:

| Plugin | Role |
|---|---|
| LengthAwarePaginator | paginator type + extra `.ts` helper file |
| LazyOptional | `Lazy` / `Optional` → optional properties |
| Hidden | drop `Hidden` properties |
| SnakeCase / SnakeCaseAttribute | property names |
| Date | Carbon/DateTime → Date / string / `S.DateFromString` |
| Collection | `Illuminate\Support\Collection` → arrays |

Contexts: `Interface`, `EncodedInterface`, `Schema`, `Enum`.

PHP: `Writer/Transformer.php`, `Plugins/*`.

### 5. Write

One `.ts` file per schema/enum, path = namespace with `\` → `/`.

Each schema file typically emits:

- TypeScript interface
- encoded interface
- Effect `S.Struct` schema

Enums: TS enum / Effect enum / const object (same three writers as PHP).

Relative imports via `OutputPathResolver` logic.

PHP: `FileWriter`, `MultiArtifactFileContentWriter`, `*SchemaWriter`, `*EnumWriter`.

### 6. CLI + Laravel wrapper

Binary: `effect-schema transform --config <toml|json> [--dry-run]`

Laravel package under `php/`: artisan `effect-schema:transform` shells out to the binary with JSON derived from `config/effect-schema.php`.

## Mago crates

- `mago-syntax` 1.46 — lexer, parser, AST walk
- `mago-names` 1.46 — FQCN resolution
- `mago-docblock` — PHPDoc AST
- `mago-database` — `File`, `FileId`
- `mago-allocator` — `LocalArena` / `Arena`
- `mago-php-version`, `mago-span` — as required by the above

Do not shell out to `mago cst`. Use the crates in-process.

## Fixtures

Copied from the PHP package: `fixtures/` (unit fixtures) and `fixtures/src/app/` (App\ Data/Enum fixtures).
