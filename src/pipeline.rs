use crate::config::Config;
use crate::error::Result;
use crate::ir::RootIr;
use crate::tokens::Token;

pub fn generate(config: &Config) -> Result<RootIr> {
    let (ir, _stats) = generate_with_stats(config)?;
    Ok(ir)
}

pub struct GenerateStats {
    pub data_classes: usize,
    pub enums: usize,
}

pub fn generate_with_stats(config: &Config) -> Result<(RootIr, GenerateStats)> {
    let data_classes = crate::discovery::discover_data_classes(config)?;
    let enums = crate::discovery::discover_enums(config)?;
    let stats = GenerateStats {
        data_classes: data_classes.len(),
        enums: enums.len(),
    };
    let mut tokens = Vec::new();

    let scanned = crate::discovery::scan_php_files(
        &config
            .data_paths
            .iter()
            .chain(config.enum_paths.iter())
            .cloned()
            .collect::<Vec<_>>(),
    )?;
    let fqcn_to_path: std::collections::BTreeMap<String, std::path::PathBuf> = scanned
        .into_iter()
        .map(|(path, fqcn)| (fqcn, path))
        .collect();

    for fqcn in &data_classes {
        if let Some(path) = fqcn_to_path.get(fqcn) {
            tokens.push(Token::Class(crate::parse::parse_data_class(fqcn, path)?));
        }
    }
    for fqcn in &enums {
        if let Some(path) = fqcn_to_path.get(fqcn) {
            tokens.push(Token::Enum(crate::parse::parse_enum(fqcn, path)?));
        }
    }

    Ok((crate::builder::build(&tokens), stats))
}

pub fn transform(config: &Config) -> Result<GenerateStats> {
    let (ir, stats) = generate_with_stats(config)?;
    let plugins = crate::plugins::load_plugins(config);
    crate::writer::write(&ir, &plugins, config)?;
    Ok(stats)
}
