//! Deskulpt windows.

mod script;

use anyhow::{anyhow, Result};
use script::WindowInitJS;
use tauri::{
    App, AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};

use crate::path::PathExt;
use crate::settings::Settings;

/// Extention trait for window-related operations.
pub trait WindowExt<R: Runtime>: Manager<R> + PathExt<R> {
    /// Create the manager and canvas windows.
    fn create_manager_and_canvas(&mut self) -> Result<()>
    where
        Self: Sized,
    {
        let initial_settings = Settings::load(self.persist_dir())?;
        let init_js = WindowInitJS::generate(initial_settings)?;

        // Create the manager window
        let manager_url = WebviewUrl::App("views/manager.html".into());
        WebviewWindowBuilder::new(self, "manager", manager_url)
            .title("Deskulpt Manager")
            .inner_size(750.0, 500.0)
            .center()
            .resizable(false)
            .maximizable(false)
            .minimizable(false)
            .visible(false)
            .initialization_script(&init_js)
            .build()?;

        // Create the canvas window
        let canvas_url = WebviewUrl::App("views/canvas.html".into());
        let canvas = WebviewWindowBuilder::new(self, "canvas", canvas_url)
            .maximized(true)
            .transparent(true)
            .decorations(false)
            .always_on_bottom(true)
            // TODO: Remove when the following issue is fixed:
            // https://github.com/tauri-apps/tauri/issues/9597
            .visible(false)
            // Unsupported on macOS; see below for activation policy
            .skip_taskbar(true)
            .initialization_script(&init_js)
            .build()?;

        #[cfg(target_os = "macos")]
        {
            use objc2::msg_send;
            use objc2::runtime::{AnyObject, Bool};

            // Disable the window shadow on macOS; there will be shadows left on
            // movement for transparent and undecorated windows that we are using;
            // it seems that disabling shadows does not have significant visual impacts
            unsafe {
                let ns_window = canvas.ns_window()? as *mut AnyObject;
                let () = msg_send![ns_window, setHasShadow:Bool::NO];
            }
        }

        // TODO: Remove when the following issue is fixed:
        // https://github.com/tauri-apps/tauri/issues/9597
        canvas.show()?;

        // Canvas is by default click-through
        canvas.set_ignore_cursor_events(true)?;

        Ok(())
    }

    /// Show the manager window.
    fn show_manager(&self) -> Result<()> {
        let manager = self
            .get_webview_window("manager")
            .ok_or(anyhow!("Manager window not found"))?;
        manager.show()?;
        manager.set_focus()?;
        Ok(())
    }
}

impl<R: Runtime> WindowExt<R> for App<R> {}
impl<R: Runtime> WindowExt<R> for AppHandle<R> {}

/// Window event handler for all Deskulpt windows.
pub fn on_window_event(window: &Window, event: &WindowEvent) {
    if window.label() == "manager" {
        // Prevent the manager window from closing when the close button is
        // clicked, but only hide it instead
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            if let Err(e) = window.hide() {
                eprintln!("Failed to hide the manager window: {}", e);
            }
        }
    }
}
