//! Deskulpt windows.
mod script;

use anyhow::Result;
use deskulpt_common::window::DeskulptWindow;
use script::{CanvasInitJS, ManagerInitJS};
use tauri::{
    App, AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};

use crate::states::SettingsStateExt;

/// Extention trait for window-related operations.
pub trait WindowExt<R: Runtime>: Manager<R> + SettingsStateExt<R> {
    /// Create the manager window.
    fn create_manager(&self) -> Result<()>
    where
        Self: Sized,
    {
        let settings = self.get_settings();
        let init_js = ManagerInitJS::generate(&settings)?;
        WebviewWindowBuilder::new(
            self,
            DeskulptWindow::Manager,
            WebviewUrl::App("src/manager/index.html".into()),
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
    fn create_canvas(&self) -> Result<()>
    where
        Self: Sized,
    {
        let settings = self.get_settings();
        let init_js = CanvasInitJS::generate(&settings)?;
        let canvas = WebviewWindowBuilder::new(
            self,
            DeskulptWindow::Canvas,
            WebviewUrl::App("src/canvas/index.html".into()),
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

    fn create_widget_window(
        &self,
        id: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<()>
    where
        Self: Sized,
    {
        let settings = self.get_settings();
        let init_js = CanvasInitJS::generate(&settings)?;

        #[cfg_attr(not(target_os = "macos"), allow(unused_variables))]
        let canvas = WebviewWindowBuilder::new(
            self,
            format!("widget-{id}"),
            WebviewUrl::App(format!("src/canvas/widget.html?id={id}").into()),
        )
        .inner_size(width.into(), height.into())
        .position(x.into(), y.into())
        .title(format!("Deskulpt Widget - {id}"))
        .maximizable(false)
        .minimizable(false)
        .closable(false)
        .transparent(true)
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

        Ok(())
    }

    /// Open the manager window.
    fn open_manager(&self) -> Result<()> {
        let manager = DeskulptWindow::Manager.webview_window(self)?;
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
