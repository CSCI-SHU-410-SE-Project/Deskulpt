//! The module includes the setup utilities of Deskulpt.

use crate::{states::CanvasClickThroughState, utils::toggle_click_through_state};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::ClickType,
    App, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};

#[cfg(target_os = "macos")]
use objc::{
    msg_send,
    runtime::{Object, NO},
    sel, sel_impl,
};

/// Create the canvas window.
pub(crate) fn create_canvas(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let canvas = WebviewWindowBuilder::new(
        app,
        "canvas",
        WebviewUrl::App("views/canvas.html".into()),
    )
    .maximized(true)
    .transparent(true)
    .decorations(false)
    .always_on_bottom(true)
    .visible(false) // TODO: https://github.com/tauri-apps/tauri/issues/9597
    .skip_taskbar(true) // Windows and Linux; macOS see below for hiding from dock
    .build()?;

    #[cfg(target_os = "macos")]
    {
        // Hide the application from the dock on macOS because hide-from-taskbar is
        // not applicable for macOS
        app.set_activation_policy(ActivationPolicy::Accessory);

        // Disable the window shadow on macOS; there will be shadows left on movement
        // for transparent and undecorated windows that we are using; it seems that
        // disabling shadows does not have significant visual impacts
        let ns_window = canvas.ns_window()? as *mut Object;
        unsafe {
            let () = msg_send![ns_window, setHasShadow:NO];
        }
    }

    canvas.show()?; // TODO: remove when `visible` is fixed

    // Be consistent with the default of `CanvasClickThroughState`
    canvas.set_ignore_cursor_events(true)?;

    Ok(())
}

/// Listen to window events.
///
/// This is to be initialized with `builder.on_window_event(listen_to_windows)` on the
/// application builder instance. It prevents the manager window from closing when the
/// close button is clicked, but only hide it instead.
pub(crate) fn listen_to_windows(window: &Window, event: &WindowEvent) {
    if window.label() == "manager" {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

/// Initialize the Deskulpt system tray.
///
/// This binds the menu and event handlers to the system tray with ID "deskulpt-tray",
/// see `tauri.conf.json`. This tray would be intialized with the following features:
///
/// - When left-clicking the tray icon or clicking the "toggle" menu item, toggle the
///   click-through state of the canvas window. Note that left-clicking is unsupported
///   on Linux, so the "toggle" menu item is present as a workaround.
/// - When clicking the "manage" menu item, show the manager window.
/// - When clicking the "exit" menu item, exit the application (with cleanup). This
///   should, in production, be the only normal way to exit the application.
pub(crate) fn init_system_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let deskulpt_tray = app.tray_by_id("deskulpt-tray").unwrap();

    // Be consistent with the default of `CanvasClickThroughState`
    let item_toggle = MenuItemBuilder::with_id("toggle", "Float").build(app)?;
    app.manage(CanvasClickThroughState::init(true, item_toggle.clone()));

    // Set up the tray menu
    let tray_menu = MenuBuilder::new(app)
        .items(&[
            &item_toggle,
            &MenuItemBuilder::with_id("manage", "Manage").build(app)?,
            &MenuItemBuilder::with_id("exit", "Exit").build(app)?,
        ])
        .build()?;
    deskulpt_tray.set_menu(Some(tray_menu))?;

    // Register event handler for the tray menu
    deskulpt_tray.on_menu_event(move |app_handle, event| match event.id().as_ref() {
        "toggle" => {
            let _ = toggle_click_through_state(app_handle); // Consume potential error
        },
        "manage" => show_manager_window(app_handle),
        "exit" => app_handle.exit(0),
        _ => {},
    });

    // Register event handler for the tray icon itself
    deskulpt_tray.on_tray_icon_event(|tray, event| {
        if event.click_type == ClickType::Left {
            let _ = toggle_click_through_state(tray.app_handle());
        }
    });

    Ok(())
}

/// Attempt to show the manager window.
///
/// This will make the manager visible if it is not already, and set focus if it is not
/// already focused. If the manager window does not exist, create the window. There is
/// no guarantee that this operation will succeed, but it will try to do so.
fn show_manager_window(app_handle: &AppHandle) {
    let inner = || -> Result<(), Box<dyn std::error::Error>> {
        if let Some(manager) = app_handle.get_webview_window("manager") {
            manager.show()?;
            manager.set_focus()?;
            return Ok(());
        }

        // Failed to get the manager window; we create a new one from the existing
        // configuration instead; note that the manager window is the first item in the
        // window list in `tauri.conf.json`
        let config = app_handle.config().app.windows.first().unwrap();
        let manager = WebviewWindowBuilder::from_config(app_handle, config)?.build()?;
        manager.show()?;
        manager.set_focus()?;
        Ok(())
    };

    let _ = inner(); // Consume any error
}
