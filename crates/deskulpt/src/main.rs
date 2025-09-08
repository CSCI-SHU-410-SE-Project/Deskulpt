#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    deskulpt::run()
}

#[cfg(test)]
mod export_bindings {
    use deskulpt_specta::TypeScript;

    #[test]
    #[ignore]
    fn export_all() {
        deskulpt::get_bindings_builder()
            .export(TypeScript::default(), "../../src/bindings.ts")
            .expect("Failed to export TypeScript bindings");
    }
}

#[cfg(test)]
mod export_schema {
    use std::fs::File;
    use std::io::BufWriter;

    use deskulpt_core::schema::Settings;
    use schemars::schema_for;

    #[test]
    #[ignore]
    fn export_settings() {
        let schema = schema_for!(Settings);
        let file = File::create("../../docs/src/public/settings-schema.json")
            .expect("Failed to create file");
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &schema).expect("Failed to write schema");
    }
}
