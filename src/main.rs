use clap::{Parser, Subcommand};
use effect_schema_generator::{transform, Config};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "effect-schema",
    about = "Generate TypeScript/Effect schemas from PHP Spatie Data classes"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate TypeScript interfaces and Effect schemas
    Transform {
        /// TOML or JSON config file
        #[arg(long, short)]
        config: Option<PathBuf>,
        /// JSON config string (used by the Laravel wrapper)
        #[arg(long)]
        config_json: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> effect_schema_generator::Result<()> {
    match cli.command {
        Commands::Transform {
            config,
            config_json,
            dry_run,
        } => {
            let cfg = if let Some(json) = config_json {
                Config::from_json_str(&json)?
            } else if let Some(path) = config {
                Config::load_file(&path)?
            } else {
                return Err(effect_schema_generator::Error::Config(
                    "pass --config or --config-json".into(),
                ));
            };
            if dry_run {
                let (ir, stats) = effect_schema_generator::generate_with_stats(&cfg)?;
                println!(
                    "Found {} data classes and {} enums",
                    stats.data_classes, stats.enums
                );
                println!("Would generate {} namespaces", ir.namespaces.len());
                for ns in ir.namespaces.values() {
                    println!("  {}", ns.namespace);
                    for schema in &ns.schemas {
                        println!("    schema {}", schema.name);
                    }
                    for enum_ir in &ns.enums {
                        println!("    enum {}", enum_ir.name);
                    }
                }
                return Ok(());
            }
            let stats = transform(&cfg)?;
            println!(
                "Found {} data classes and {} enums",
                stats.data_classes, stats.enums
            );
            println!("Schema generation completed successfully!");
            Ok(())
        }
    }
}
