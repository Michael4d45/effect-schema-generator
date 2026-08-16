pub mod builder;
pub mod config;
pub mod discovery;
pub mod error;
pub mod ir;
pub mod parse;
pub mod pipeline;
pub mod plugins;
pub mod test_util;
pub mod tokens;
pub mod writer;

pub use config::{Config, TypeMap};
pub use error::{Error, Result};
pub use ir::RootIr;
pub use pipeline::{generate, generate_with_stats, transform, GenerateStats};
