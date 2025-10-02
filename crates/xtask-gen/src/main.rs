mod bindings;

use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser, ValueEnum)]
enum GenKind {
    Bindings,
}

#[derive(Debug, Parser)]
struct Args {
    kind: GenKind,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.kind {
        GenKind::Bindings => bindings::run()?,
    }
    Ok(())
}
