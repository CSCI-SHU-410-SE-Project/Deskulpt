//! The module includes the setup utilities of Deskulpt.

use std::thread::{sleep, spawn};
use std::time::Duration;

#[cfg(target_os = "macos")]
use objc::{
    msg_send,
    runtime::{Object, NO},
    sel, sel_impl,
};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};
use tauri::{
    App, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};

use crate::states::CanvasClickThroughState;
use crate::utils::toggle_click_through_state;

/// Create the canvas window.
pub(crate) fn create_canvas(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let canvas =
        WebviewWindowBuilder::new(app, "canvas", WebviewUrl::App("views/canvas.html".into()))
            .maximized(true)
            .transparent(true)
            .decorations(false)
            .always_on_bottom(true)
            .visible(false) // TODO: https://github.com/tauri-apps/tauri/issues/9597
            .skip_taskbar(true) // Windows and Linux; macOS see below for hiding from dock
            .build()?;

    #[cfg(target_os = "macos")]
    // Disable the window shadow on macOS; there will be shadows left on movement for
    // transparent and undecorated windows that we are using; it seems that disabling
    // shadows does not have significant visual impacts
    unsafe {
        let ns_window = canvas.ns_window()? as *mut Object;
        let () = msg_send![ns_window, setHasShadow:NO];
    }

    canvas.show()?; // TODO: remove when `visible` is fixed

    // Be consistent with the default of `CanvasClickThroughState`
    canvas.set_ignore_cursor_events(true)?;

    Ok(())
}

/// Listen to window events.
///
/// This is to be initialized with `builder.on_window_event(listen_to_windows)`
/// on the application builder instance. It prevents the manager window from
/// closing when the close button is clicked, but only hide it instead.
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
/// This binds the menu and event handlers to the system tray with ID
/// "deskulpt-tray", see `tauri.conf.json`. Note that the cnavas click-through
/// state is managed in this function as well! This tray would be intialized
/// with the following features:
///
/// - When left-clicking the tray icon or clicking the "toggle" menu item,
///   toggle the click-through state of the canvas window. Note that
///   left-clicking is unsupported on Linux, so the "toggle" menu item is
///   present as a workaround.
/// - When clicking the "manage" menu item, show the manager window.
/// - When clicking the "exit" menu item, exit the application (with cleanup).
///   This should, in production, be the only normal way to exit the
///   application.
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
            let _ = toggle_click_through_state(app_handle); // Consume potential
                                                            // error
        },
        "manage" => show_manager_window(app_handle),
        "exit" => on_app_exit(app_handle),
        _ => {},
    });

    // Register event handler for the tray icon itself
    deskulpt_tray.on_tray_icon_event(|tray, event| {
        if let TrayIconEvent::Click {
            button,
            button_state,
            ..
        } = event
        {
            if button == MouseButton::Left && button_state == MouseButtonState::Down {
                let _ = toggle_click_through_state(tray.app_handle()); // Consume error
            }
        }
    });

    Ok(())
}

/// Attempt to show the manager window.
///
/// This will make the manager visible if it is not already, and set focus if it
/// is not already focused. If the manager window does not exist, create the
/// window. There is no guarantee that this operation will succeed, but it will
/// try to do so.
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

/// The cleanup function to be called on application exit.
fn on_app_exit(app_handle: &AppHandle) {
    if app_handle.get_webview_window("manager").is_none() {
        app_handle.exit(0); // Manager window does not exist; should not happen
    };

    // Emit the "exit-app" event to the manager window so that it can save the
    // global settings to a file before the application exits; it will then be
    // in charge of exiting the application
    if app_handle.emit_to("manager", "exit-app", ()).is_err() {
        app_handle.exit(0); // Event fails to be emitted
    }

    // This is a safeguard to ensure that the application exits in case the manager
    // window fails to do so; we give it a 5-second timeout
    let app_handle = app_handle.clone();
    spawn(move || {
        sleep(Duration::from_secs(5));
        app_handle.exit(0);
    });
}
