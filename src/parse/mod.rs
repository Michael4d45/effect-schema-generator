use std::borrow::Cow;
use std::collections::BTreeMap;

use mago_allocator::LocalArena;
use mago_database::file::File;
use mago_syntax::parser::parse_file;

use crate::error::Result;

mod class;
mod enums;
mod names;
mod phpdoc;
mod types;
mod uses;

pub use class::parse_data_class;
pub use enums::parse_enum;
pub use phpdoc::phpdoc_type_string;

/// Extract `use` alias → FQCN map from PHP source.
pub fn parse_use_statements(source: &str) -> Result<BTreeMap<String, String>> {
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"uses.php"),
        Cow::Owned(source.as_bytes().to_vec()),
    );
    let program = parse_file(&arena, &file);
    Ok(uses::collect_use_map(&program.statements))
}
