#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

use deskulpt_core::path::PathExt;
use deskulpt_core::states::{
    CanvasImodeStateExt, InitialRenderStateExt, SettingsStateExt, WidgetConfigMapStateExt,
};
use deskulpt_core::tray::TrayExt;
use deskulpt_core::window::WindowExt;
use tauri::image::Image;
use tauri::{generate_context, include_image, Builder, Wry};
use tauri_specta::{collect_commands, collect_events};

/// Image object for the Deskulpt icon.
const DESKULPT_ICON: Image = include_image!("./icons/icon.png");

/// Get the builder of bindings for Deskulpt commands and events.
pub fn get_bindings_builder() -> tauri_specta::Builder {
    tauri_specta::Builder::<Wry>::new()
        .commands(collect_commands![
            deskulpt_core::commands::bundle_widget::<Wry>,
            deskulpt_core::commands::call_plugin::<Wry>,
            deskulpt_core::commands::emit_on_render_ready::<Wry>,
            deskulpt_core::commands::exit_app::<Wry>,
            deskulpt_core::commands::open_widget::<Wry>,
            deskulpt_core::commands::rescan_widgets::<Wry>,
            deskulpt_core::commands::set_render_ready::<Wry>,
            deskulpt_core::commands::update_settings::<Wry>,
        ])
        .events(collect_events![
            deskulpt_core::events::ExitAppEvent,
            deskulpt_core::events::RemoveWidgetsEvent,
            deskulpt_core::events::RenderWidgetsEvent,
            deskulpt_core::events::ShowToastEvent,
            deskulpt_core::events::SwitchThemeEvent,
            deskulpt_core::events::UpdateSettingsEvent,
        ])
        .typ::<deskulpt_core::window::DeskulptWindow>()
}

/// Entry point for the Deskulpt backend.
pub fn run() {
    let bindings_builder = get_bindings_builder();

    Builder::default()
        .invoke_handler(bindings_builder.invoke_handler())
        .setup(move |app| {
            bindings_builder.mount_events(app);

            app.init_widgets_dir()?;
            app.init_persist_dir()?;

            app.manage_settings();
            app.manage_initial_render();
            app.manage_widget_config_map();
            app.manage_canvas_imode();

            // Hide the application from the dock on macOS because skipping
            // taskbar is not applicable for macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            app.create_manager()?;
            app.create_canvas()?;
            app.create_tray(DESKULPT_ICON)?;

            Ok(())
        })
        .on_window_event(deskulpt_core::window::on_window_event)
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
