use tauri_build::{
    try_build, AppManifest, Attributes, CodegenContext, InlinedPlugin, WindowsAttributes,
};

const INTERNAL_COMMANDS: &[&str] = &[
    "bundle_widget",
    "exit_app",
    "init_global_setting",
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
            .app_manifest(AppManifest::new().commands(INTERNAL_COMMANDS))
            // Workaround for `STATUS_ENTRYPOINT_NOT_FOUND` in tests
            // https://github.com/tauri-apps/tauri/discussions/11179
            .windows_attributes(WindowsAttributes::new_without_app_manifest()),
    )
    .expect("Failed to run tauri-build");

    // Workaround for `STATUS_ENTRYPOINT_NOT_FOUND` in tests
    // https://github.com/tauri-apps/tauri/discussions/11179
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV");
    let is_deskulpt_workspace =
        std::env::var("__DESKULPT_WORKSPACE__").map_or(false, |v| v == "true");
    if is_deskulpt_workspace && target_os == "windows" && Ok("msvc") == target_env.as_deref() {
        let manifest = std::env::current_dir()
            .unwrap()
            .join("windows-manifest.xml");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!(
            "cargo:rustc-link-arg=/MANIFESTINPUT:{}",
            manifest.to_str().unwrap()
        );
        println!("cargo:rustc-link-arg=/WX");
    }
}
