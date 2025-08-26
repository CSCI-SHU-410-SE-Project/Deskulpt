#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

use deskulpt_core::{
    PathExt, StatesExtInitialRender, StatesExtSettings, StatesExtWidgetConfigMap, TrayExt,
    WindowExt,
};
use tauri::image::Image;
use tauri::{generate_context, generate_handler, include_image, Builder};

/// Image object for the Deskulpt icon.
const DESKULPT_ICON: Image = include_image!("./icons/icon.png");

/// Entry point for the Deskulpt backend.
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.init_widgets_dir()?;
            app.init_persist_dir()?;

            app.manage_initial_render();
            app.manage_settings();
            app.manage_widget_config_map();

            // Hide the application from the dock on macOS because skipping
            // taskbar is not applicable for macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            app.create_manager()?;
            app.create_canvas()?;
            app.create_tray(DESKULPT_ICON)?;

            Ok(())
        })
        .on_window_event(deskulpt_core::on_window_event)
        .invoke_handler(generate_handler![
            deskulpt_core::commands::call_plugin,
            deskulpt_core::commands::bundle_widget,
            deskulpt_core::commands::emit_on_render_ready,
            deskulpt_core::commands::exit_app,
            deskulpt_core::commands::open_widget,
            deskulpt_core::commands::rescan_widgets,
            deskulpt_core::commands::set_render_ready,
            deskulpt_core::commands::update_settings,
        ])
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Prevent the opener plugin from registering handler for click event
        // so we can register our own that opens non-_blank anchors in new tab
        .plugin(
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
        .run(generate_context!())
        .expect("Error running the Deskulpt application");
}
