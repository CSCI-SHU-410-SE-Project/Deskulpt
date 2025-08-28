//! Deskulpt system tray.

use std::time::Duration;

use anyhow::Result;
use tauri::image::Image;
use tauri::menu::{
    CheckMenuItem, CheckMenuItemBuilder, MenuBuilder, MenuEvent, MenuItemBuilder, SubmenuBuilder,
};
use tauri::tray::TrayIconBuilder;
use tauri::{App, AppHandle, Runtime};
use tokio::time::sleep;

use crate::events::{DeskulptEvent, ExitAppEvent};
use crate::settings::{CanvasImode, SettingsUpdate};
use crate::states::StatesExtSettings;
use crate::window::{DeskulptWindow, WindowExt};

/// System tray menu items that may change at runtime.
pub struct MenuItems<R: Runtime> {
    pub canvas_imode_sink: CheckMenuItem<R>,
    pub canvas_imode_float: CheckMenuItem<R>,
}

impl<R: Runtime> MenuItems<R> {
    /// Update canvas interaction mode menu items according to the given mode.
    pub fn set_canvas_imode(&self, mode: CanvasImode) -> Result<()> {
        match mode {
            CanvasImode::Float => {
                self.canvas_imode_float.set_checked(true)?;
                self.canvas_imode_sink.set_checked(false)?;
            },
            CanvasImode::Sink => {
                self.canvas_imode_float.set_checked(false)?;
                self.canvas_imode_sink.set_checked(true)?;
            },
        }
        Ok(())
    }
}

/// Extention trait for system tray-related operations.
pub trait TrayExt<R: Runtime>: StatesExtSettings<R> {
    /// Create the system tray.
    fn create_tray(&self, icon: Image) -> Result<MenuItems<R>>
    where
        Self: Sized,
    {
        // Build the canvas interaction mode submenu
        let canvas_imode = &self.get_readable_settings().app.canvas_imode;
        let menu_item_canvas_imode_sink =
            CheckMenuItemBuilder::with_id("tray-canvas-imode-sink", "Sink")
                .checked(canvas_imode == &CanvasImode::Sink)
                .build(self)?;
        let menu_item_canvas_imode_float =
            CheckMenuItemBuilder::with_id("tray-canvas-imode-float", "Float")
                .checked(canvas_imode == &CanvasImode::Float)
                .build(self)?;
        let menu_item_canvas_imode = SubmenuBuilder::with_id(self, "tray-canvas-imode", "Canvas")
            .item(&menu_item_canvas_imode_sink)
            .item(&menu_item_canvas_imode_float)
            .build()?;

        // Build the system tray menu
        let tray_menu = MenuBuilder::new(self)
            .items(&[
                &menu_item_canvas_imode,
                &MenuItemBuilder::with_id("tray-manage", "Manage").build(self)?,
                &MenuItemBuilder::with_id("tray-exit", "Exit").build(self)?,
            ])
            .build()?;

        // Build the system tray icon
        TrayIconBuilder::with_id("tray")
            .icon(icon)
            .icon_as_template(true)
            .tooltip("Deskulpt")
            .menu(&tray_menu)
            .on_menu_event(on_menu_event)
            .build(self)?;

        Ok(MenuItems {
            canvas_imode_sink: menu_item_canvas_imode_sink,
            canvas_imode_float: menu_item_canvas_imode_float,
        })
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
        "tray-canvas-imode-sink" => {
            if let Err(e) =
                app_handle.update_settings([SettingsUpdate::canvas_imode(CanvasImode::Sink)])
            {
                eprintln!("Error toggling canvas interaction mode: {e}");
            }
        },
        "tray-canvas-imode-float" => {
            if let Err(e) =
                app_handle.update_settings([SettingsUpdate::canvas_imode(CanvasImode::Float)])
            {
                eprintln!("Error toggling canvas interaction mode: {e}");
            }
        },
        "tray-manage" => {
            if let Err(e) = app_handle.open_manager() {
                eprintln!("Error opening manager window: {e}");
            }
        },
        "tray-exit" => {
            // Emit the "exit-app" event to the manager, which will invoke the
            // `exit_app` command to clean up and actually exit the application
            if let Err(e) = ExitAppEvent.emit_to(app_handle, DeskulptWindow::Manager) {
                eprintln!("Failed to emit exit-app to manager: {e}");
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
