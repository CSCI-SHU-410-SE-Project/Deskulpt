use tauri_build::{try_build, AppManifest, Attributes, CodegenContext, InlinedPlugin};

const INTERNAL_COMMANDS: &[&str] = &[
    "bundle_widget",
    "exit_app",
    "init_settings",
    "open_widget_resource",
    "refresh_widget_collection",
    "register_toggle_shortcut",
];

const WIDGET_APIS_FS_COMMANDS: &[&str] = &[
    "is_file",
    "is_dir",
    "exists",
    "read_file",
    "write_file",
    "append_file",
    "remove_file",
    "create_dir",
    "remove_dir",
];

const WIDGET_APIS_SYS_COMMANDS: &[&str] = &["get_system_info"];

fn main() {
    try_build(
        Attributes::new()
            .codegen(CodegenContext::new())
            .plugin(
                "apis-fs",
                InlinedPlugin::new().commands(WIDGET_APIS_FS_COMMANDS),
            )
            .plugin(
                "apis-sys",
                InlinedPlugin::new().commands(WIDGET_APIS_SYS_COMMANDS),
            )
            .app_manifest(AppManifest::new().commands(INTERNAL_COMMANDS)),
    )
    .expect("Failed to run tauri-build");
}
