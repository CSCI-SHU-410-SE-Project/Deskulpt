fn main() {
    // Workaround for `STATUS_ENTRYPOINT_NOT_FOUND` in tests
    // https://github.com/tauri-apps/tauri/discussions/11179
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV");
    let is_deskulpt_workspace =
        std::env::var("__DESKULPT_WORKSPACE__").map_or(false, |v| v == "true");
    if is_deskulpt_workspace && target_os == "windows" && Ok("msvc") == target_env.as_deref() {
        let manifest = std::env::current_dir()
            .unwrap()
            .join("../deskulpt/windows-manifest.xml");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!(
            "cargo:rustc-link-arg=/MANIFESTINPUT:{}",
            manifest.to_str().unwrap()
        );
        println!("cargo:rustc-link-arg=/WX");
    }
}
