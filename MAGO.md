# Mago crate notes (in-process, not the CLI)

Version pin: `mago-syntax` / `mago-names` / `mago-database` / `mago-allocator` **1.46**. `mago-docblock` **1.42**.

## Parse a file

```rust
use mago_allocator::prelude::*;
use mago_database::file::File;
use mago_syntax::parser::parse_file;

let arena = LocalArena::new();
let file = File::read(workspace, path, mago_database::file::FileType::Host)
    .expect("read");
let program = parse_file(&arena, &file);
```

In-memory:

```rust
let file = File::ephemeral("t.php".into(), source.as_bytes().to_vec().into());
let program = parse_file(&arena, &file);
```

`Program` is arena-allocated. Walk it and copy what you need into owned Rust structs (`ClassToken`, etc.). Do not store `&Program` past the arena.

## Walk

`program.statements` is a sequence of statement nodes. Look for `Namespace`, `Class`, `Enum`, `Use` (and statements nested in namespace bodies).

Promoted constructor properties live on the constructor method parameters with visibility.

Trivia/docblocks sit on the program or adjacent nodes; parse with `mago_docblock::parse_phpdoc_with_span` / `parse_trivia`.

## Names

`mago-names` resolves `Carbon` → `Carbon\Carbon` given the file's `use` list. Prefer it over hand-rolled import maps when possible; still fill `ClassToken.uses` (alias → FQCN) because the builder needs that map for PHPDoc identifiers.

## Data class check

A class is Spatie Data if any `extends` name resolves to `Spatie\LaravelData\Data`. Walk parent classes in other files when the extends name is another project class.
