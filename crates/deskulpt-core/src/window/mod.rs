//! Deskulpt windows.
mod script;

use anyhow::Result;
use script::{CanvasInitJS, ManagerInitJS};
use tauri::{
    App, AppHandle, Manager, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window,
    WindowEvent,
};

use crate::settings::Settings;

#[derive(ts_rs::TS)]
#[ts(rename_all = "lowercase", export, export_to = "types.ts")]
pub enum DeskulptWindow {
    Manager,
    Canvas,
}

impl DeskulptWindow {
    /// Get the label of the window.
    pub fn label(&self) -> &str {
        match self {
            DeskulptWindow::Manager => "manager",
            DeskulptWindow::Canvas => "canvas",
        }
    }

    /// Get the URL of the window.
    pub fn url(&self) -> WebviewUrl {
        match self {
            DeskulptWindow::Manager => WebviewUrl::App("manager/index.html".into()),
            DeskulptWindow::Canvas => WebviewUrl::App("canvas/index.html".into()),
        }
    }

    /// Get the webview window.
    ///
    /// This method panics if the window is not found, but this should not
    /// happen.
    pub fn webview_window<R, M>(&self, manager: &M) -> WebviewWindow<R>
    where
        R: Runtime,
        M: Manager<R> + ?Sized,
    {
        manager
            .get_webview_window(self.label())
            .expect(&format!("Window not found: {}", self.label()))
    }
}

/// Extention trait for window-related operations.
pub trait WindowExt<R: Runtime>: Manager<R> {
    /// Create the manager window.
    fn create_manager(&mut self, settings: &Settings) -> Result<()>
    where
        Self: Sized,
    {
        let init_js = ManagerInitJS::generate(settings)?;
        WebviewWindowBuilder::new(
            self,
            DeskulptWindow::Manager.label(),
            DeskulptWindow::Manager.url(),
        )
        .title("Deskulpt Manager")
        .inner_size(800.0, 500.0)
        .center()
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .visible(false)
        .initialization_script(&init_js)
        .build()?;

        Ok(())
    }

    /// Create the canvas window.
    fn create_canvas(&mut self, settings: &Settings) -> Result<()>
    where
        Self: Sized,
    {
        let init_js = CanvasInitJS::generate(settings)?;
        let canvas = WebviewWindowBuilder::new(
            self,
            DeskulptWindow::Canvas.label(),
            DeskulptWindow::Canvas.url(),
        )
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

    /// Open the manager window.
    fn open_manager(&self) -> Result<()> {
        let manager = DeskulptWindow::Manager.webview_window(self);
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
                eprintln!("Failed to hide the manager window: {e}");
            }
        }
    }
}
