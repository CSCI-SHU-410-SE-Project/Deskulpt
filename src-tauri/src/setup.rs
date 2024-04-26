//! The module configures the system tray of Deskulpt.

use tauri::{
    App, AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTray,
    SystemTrayEvent, SystemTrayMenu, WindowBuilder, WindowEvent, WindowUrl,
};

#[cfg(target_os = "macos")]
use objc::{msg_send, runtime::Object, sel, sel_impl};

#[cfg(target_os = "macos")]
extern "C" {
    fn CGWindowLevelForKey(key: i32) -> i32;
}

pub(crate) fn create_canvas(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let canvas =
        WindowBuilder::new(app, "canvas", WindowUrl::App("views/canvas.html".into()))
            .maximized(true)
            .transparent(true)
            .decorations(false)
            .build()?;

    #[cfg(target_os = "macos")]
    {
        let ns_window = canvas.ns_window()? as *mut Object;
        unsafe {
            let () = msg_send![ns_window, setLevel:CGWindowLevelForKey(18)];
        }
    }

    Ok(())
}

/// Listen to global window events.
///
/// This is to be initialized with `builder.on_window_event(listen_to_windows)` on the
/// application builder instance. It does the following:
///
/// - Prevent the manager window from closing when the close button is clicked.
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
        .add_item(CustomMenuItem::new("focus", "Focus"))
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
/// - When clicking the "exit" menu item, exit the application (with cleanup).
pub(crate) fn listen_to_system_tray(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "manage" => show_manager_window(app_handle),
            "focus" => focus_canvas_window(app_handle),
            "exit" => app_handle.exit(0),
            _ => {},
        },
        SystemTrayEvent::LeftClick { .. } => {
            focus_canvas_window(app_handle);
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
        let config = app_handle.config().tauri.windows.get(0).unwrap().clone();
        // Discard any error if the window fails to be built, because this likely means
        // that the manager window is still there
        WindowBuilder::from_config(app_handle, config.clone()).build().ok()
    }) {
        let _ = manager.show(); // Discard any error
    }
}

fn focus_canvas_window(app_handle: &AppHandle) {
    let canvas =
        app_handle.get_window("canvas").expect("Failed to get the canvas window");
    canvas.set_focus().expect("Failed to focus on the canvas window");
}
