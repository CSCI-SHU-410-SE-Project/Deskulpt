#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg"
)]

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{generate_handler, Runtime};

mod bundler;
mod commands;
mod config;
pub mod events;
pub mod path;
mod settings;
pub mod states;
pub mod tray;
pub mod window;

/// Initialize the `deskulpt-core` plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("deskulpt-core")
        .invoke_handler(generate_handler![
            commands::bundle_widget,
            commands::call_plugin,
            commands::emit_on_render_ready,
            commands::exit_app,
            commands::open_widget,
            commands::rescan_widgets,
            commands::set_render_ready,
            commands::update_settings,
        ])
        .build()
}

/// Re-exports for JSON schema generation.
pub mod schema {
    pub use crate::settings::Settings;
}

#[doc(hidden)]
pub fn configure_bindings_builder(builder: &mut deskulpt_common::bindings::BindingsBuilder) {
    builder
        .commands(
            "deskulpt-core",
            deskulpt_common::bindings::collect_commands![
                commands::bundle_widget::<tauri::Wry>,
                commands::call_plugin::<tauri::Wry>,
                commands::emit_on_render_ready::<tauri::Wry>,
                commands::exit_app::<tauri::Wry>,
                commands::open_widget::<tauri::Wry>,
                commands::rescan_widgets::<tauri::Wry>,
                commands::set_render_ready::<tauri::Wry>,
                commands::update_settings::<tauri::Wry>,
            ],
        )
        .event::<events::ExitAppEvent>()
        .event::<events::RemoveWidgetsEvent>()
        .event::<events::RenderWidgetsEvent>()
        .event::<events::ShowToastEvent>()
        .event::<events::SwitchThemeEvent>()
        .event::<events::UpdateSettingsEvent>();
}
