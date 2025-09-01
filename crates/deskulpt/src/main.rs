#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    deskulpt::run()
}

#[cfg(test)]
mod export_bindings {
    use deskulpt_specta::TypeScript;

    #[test]
    fn main() {
        deskulpt::get_bindings_builder()
            .export(TypeScript::default(), "../../src/bindings.ts")
            .expect("Failed to export TypeScript bindings");
    }
}
