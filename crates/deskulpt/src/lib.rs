#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

use deskulpt_core::{
    PathExt, Settings, StatesExtCanvasClickThrough, StatesExtWidgetCollection, TrayExt, WindowExt,
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

            let settings = match Settings::load(app.persist_dir()) {
                Ok(settings) => settings,
                Err(e) => {
                    eprintln!("Failed to load settings: {e}");
                    Settings::default()
                },
            };

            app.manage_widget_collection();
            app.manage_canvas_click_through();

            // Hide the application from the dock on macOS because skipping
            // taskbar is not applicable for macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            app.create_manager(&settings)?;
            app.create_canvas(&settings)?;

            app.create_tray(DESKULPT_ICON)?;

            Ok(())
        })
        .on_window_event(deskulpt_core::on_window_event)
        .invoke_handler(generate_handler![
            deskulpt_core::commands::call_plugin,
            deskulpt_core::commands::bundle_widget,
            deskulpt_core::commands::exit_app,
            deskulpt_core::commands::open_in_widgets_dir,
            deskulpt_core::commands::rescan_widgets,
            deskulpt_core::commands::update_toggle_shortcut,
        ])
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .run(generate_context!())
        .expect("Error running the Deskulpt application");
}
