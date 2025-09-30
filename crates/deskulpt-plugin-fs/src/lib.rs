#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg"
)]

mod commands;

use deskulpt_plugin::{register_commands, Plugin};

/// The file system plugin (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// Redesign the exposed APIs, possibly referring to the APIs of the
/// [Tauri file system plugin](https://v2.tauri.app/plugin/file-system).
pub struct FsPlugin;

impl Plugin for FsPlugin {
    register_commands![
        commands::AppendFile,
        commands::CreateDir,
        commands::Exists,
        commands::IsDir,
        commands::IsFile,
        commands::ReadFile,
        commands::RemoveDir,
        commands::RemoveFile,
        commands::WriteFile,
    ];
}
