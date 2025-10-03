const COMMANDS: &[&str] = &[
    "bundle_widget",
    "call_plugin",
    "emit_on_render_ready",
    "exit_app",
    "open_widget",
    "rescan_widgets",
    "set_render_ready",
    "update_settings",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
