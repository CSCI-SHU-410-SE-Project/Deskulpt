use anyhow::Result;
use schemars::schema_for;

/// Entry point for the `cargo gen schema` command.
pub fn run() -> Result<()> {
    let schema = schema_for!(deskulpt_core::schema::Settings);
    let output = serde_json::to_string_pretty(&schema)?;

    let path = deskulpt_workspace::docs_dir().join("src/public/settings-schema.json");
    std::fs::write(&path, output)?;
    println!("✅ Generated: {}", path.display());

    Ok(())
}
