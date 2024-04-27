//! The module configures the system tray of Deskulpt.

use std::{thread::sleep, time::Duration};

use tauri::{
    AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, WindowBuilder, WindowEvent,
};

/// Listen to global window events.
///
/// This is to be initialized with `builder.on_window_event(listen_to_windows)` on the
/// application builder instance. It does the following:
///
/// - Prevent the manager window from closing when the close button is clicked but hide
///   it instead.
pub(crate) fn listen_to_windows(e: GlobalWindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = e.event() {
        let window = e.window();
        if window.label() == "manager" {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

/// Get the system tray of Deskulpt.
///
/// This is to be initialized with `builder.system_tray(get_system_tray())` on the
/// application builder instance.
pub(crate) fn get_system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("manage", "Manage"))
        .add_item(CustomMenuItem::new("exit", "Exit"));
    SystemTray::new().with_menu(tray_menu)
}

/// Listen to system tray events.
///
/// This is to be initialized with `builder.on_system_tray_event(listen_to_system_tray)`
/// on the application builder instance. It does the following:
///
/// - When left-clicking the tray icon or clicking the "manage" menu item, show the
///   manager window. Note that left-clicking is unsupported on Linux, so the "manage"
///   menu item is present as a workaround.
/// - When clicking the "exit" menu item, exit the application (with cleanup). This
///   should, in production, be the only normal way to exit the application.
pub(crate) fn listen_to_system_tray(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "manage" => show_manager_window(app_handle),
            "exit" => on_app_exit(app_handle),
            _ => {},
        },
        SystemTrayEvent::LeftClick { .. } => {
            show_manager_window(app_handle);
        },
        _ => {},
    }
}

/// Attempt to show the manager window.
///
/// If the manager window does not exist, create the window. If the window exists but
/// fails to show, consume the error and do nothing.
fn show_manager_window(app_handle: &AppHandle) {
    if let Some(manager) = app_handle.get_window("manager").or_else(|| {
        // Failed to get the manager window; we create a new one from the existing
        // configuration instead; note that the manager window is the second item in
        // the window list in `tauri.conf.json5`
        let config = app_handle.config().tauri.windows.get(1).unwrap().clone();
        // Discard any error if the window fails to be built, because this likely means
        // that the manager window is still there
        WindowBuilder::from_config(app_handle, config.clone()).build().ok()
    }) {
        let _ = manager.show(); // Discard any error
    }
}

/// The cleanup function to be called on application exit.
fn on_app_exit(app_handle: &AppHandle) {
    if app_handle.get_window("canvas").is_none() {
        // Exit immediately if the canvas window does not exist
        app_handle.exit(0);
    };

    // Emit the "exit-app" event to the canvas window so that it can save the widget
    // internals to a file before the application exists; it will be in charge of
    // exiting the application
    if app_handle.emit_to("canvas", "exit-app", ()).is_err() {
        // Exit immediately if the event fails to be emitted
        app_handle.exit(0);
    }

    // This is a safeguard to ensure that the application exists in case the canvas
    // window fails to do so; we give it 5 seconds to exit
    sleep(Duration::from_secs(5));
    app_handle.exit(0);
}
