//! Deskulpt system tray.

use std::time::Duration;

use anyhow::Result;
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuEvent, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Runtime};
use tokio::time::sleep;

use crate::events::EventsExt;
use crate::states::StatesExtCanvasImode;
use crate::window::WindowExt;

/// Extention trait for system tray-related operations.
pub trait TrayExt<R: Runtime>: StatesExtCanvasImode<R> {
    /// Create the system tray.
    fn create_tray(&self, icon: Image) -> Result<()>
    where
        Self: Sized,
    {
        // Store the menu item for toggling canvas interaction mode
        let menu_item_toggle = MenuItemBuilder::with_id("tray-toggle", "Float").build(self)?;
        self.set_canvas_imode_menu_item(&menu_item_toggle);

        // Build the system tray menu
        let tray_menu = MenuBuilder::new(self)
            .items(&[
                &menu_item_toggle,
                &MenuItemBuilder::with_id("tray-manage", "Manage").build(self)?,
                &MenuItemBuilder::with_id("tray-exit", "Exit").build(self)?,
            ])
            .build()?;

        // Build the system tray icon
        TrayIconBuilder::with_id("tray")
            .icon(icon)
            .icon_as_template(true)
            .show_menu_on_left_click(false)
            .tooltip("Deskulpt")
            .menu(&tray_menu)
            .on_menu_event(on_menu_event)
            .on_tray_icon_event(on_tray_icon_event)
            .build(self)?;

        Ok(())
    }
}

impl<R: Runtime> TrayExt<R> for App<R> {}
impl<R: Runtime> TrayExt<R> for AppHandle<R> {}

/// Handler for system tray menu events.
///
/// This handler will receive any menu event but only act on events related to
/// the system tray.
fn on_menu_event<R: Runtime>(app_handle: &AppHandle<R>, event: MenuEvent) {
    match event.id().as_ref() {
        "tray-toggle" => {
            if let Err(e) = app_handle.toggle_canvas_imode() {
                eprintln!("Error toggling canvas interaction mode: {}", e);
            }
        },
        "tray-manage" => {
            if let Err(e) = app_handle.open_manager() {
                eprintln!("Error opening manager window: {}", e);
            }
        },
        "tray-exit" => {
            // Emit the "exit-app" event to the manager, which will invoke the
            // `exit_app` command to clean up and actually exit the application
            if let Err(e) = app_handle.emit_exit_app_to_manager() {
                eprintln!("Failed to emit exit-app to manager: {}", e);
                app_handle.exit(1); // Safeguard exit
            }

            // Safeguard exit after 5 seconds if the normal exit procedure fails
            let app_handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_secs(5)).await;
                app_handle.exit(1);
            });
        },
        _ => {},
    }
}

/// Handler for system tray icon events.
fn on_tray_icon_event<R: Runtime>(tray: &TrayIcon<R>, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button,
        button_state,
        ..
    } = event
    {
        if button == MouseButton::Left && button_state == MouseButtonState::Down {
            // Toggle canvas interaction mode on left-click
            if let Err(e) = tray.app_handle().toggle_canvas_imode() {
                eprintln!("Error toggling canvas interaction mode: {}", e);
            }
        }
    }
}
