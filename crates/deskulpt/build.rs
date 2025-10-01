fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "bundle_widget",
            "call_plugin",
            "emit_on_render_ready",
            "exit_app",
            "open_widget",
            "rescan_widgets",
            "set_render_ready",
            "update_settings",
        ]),
    ))
    .unwrap_or_else(|e| panic!("Error during tauri-build: {e:?}"));
}
