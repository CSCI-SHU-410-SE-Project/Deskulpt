mod bindings;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate Deskulpt frontend bindings.
    Bindings,
}

/// [XTASK] Code generation for Deskulpt.
#[derive(Debug, Parser)]
#[command(version, about, author, bin_name = "cargo gen")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Bindings => bindings::run()?,
    }
    Ok(())
}
