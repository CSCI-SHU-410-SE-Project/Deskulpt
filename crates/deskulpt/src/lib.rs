#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

use deskulpt_core::{
    PathExt, StatesExtCanvasClickThrough, StatesExtWidgetCollection, TrayExt, WindowExt,
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

            app.manage_widget_collection();
            app.manage_canvas_click_through();

            app.create_manager()?;
            app.create_canvas()?;

            app.create_tray(DESKULPT_ICON)?;

            Ok(())
        })
        .on_window_event(deskulpt_core::on_window_event)
        .invoke_handler(generate_handler![
            deskulpt_core::commands::call_plugin,
            deskulpt_core::commands::bundle_widget,
            deskulpt_core::commands::exit_app,
            deskulpt_core::commands::load_settings,
            deskulpt_core::commands::open_in_widgets_dir,
            deskulpt_core::commands::rescan_widgets,
            deskulpt_core::commands::update_toggle_shortcut,
        ])
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .run(generate_context!())
        .expect("Error running the Deskulpt application");
}
