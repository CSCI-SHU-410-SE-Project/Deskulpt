//! Deskulpt windows.

use anyhow::{anyhow, Result};
use tauri::{
    App, AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};

/// Initialization script for the canvas window.
const CANVAS_INIT_SCRIPT: &str = concat!(
    "Object.defineProperty(window, \"__DESKULPT_INTERNALS__\", { value: { ",
    "apisTemplate: `",
    include_str!("../.scripts/apis.template.js"),
    "` } });",
);

/// Extention trait for window-related operations.
pub trait WindowExt {
    /// Create the manager window.
    fn create_manager(&mut self) -> Result<()>;

    /// Create the canvas window.
    fn create_canvas(&mut self) -> Result<()>;

    /// Show the manager window.
    fn show_manager(&self) -> Result<()>;
}

/// Shared implementation of [`WindowExt`].
macro_rules! shared_impl {
    ($app: ty) => {
        impl<R: Runtime> WindowExt for $app {
            fn create_manager(&mut self) -> Result<()> {
                WebviewWindowBuilder::new(self, "manager", WebviewUrl::App("views/manager.html".into()))
                    .title("Deskulpt Manager")
                    .inner_size(750.0, 500.0)
                    .center()
                    .resizable(false)
                    .maximizable(false)
                    .minimizable(false)
                    .visible(false)
                    .build()?;

                Ok(())
            }

            fn create_canvas(&mut self) -> Result<()> {
                let canvas = WebviewWindowBuilder::new(self, "canvas", WebviewUrl::App("views/canvas.html".into()))
                    .maximized(true)
                    .transparent(true)
                    .decorations(false)
                    .always_on_bottom(true)
                    // TODO: Remove when the following issue is fixed:
                    // https://github.com/tauri-apps/tauri/issues/9597
                    .visible(false)
                    // Unsupported on macOS; see below for activation policy
                    .skip_taskbar(true)
                    .initialization_script(CANVAS_INIT_SCRIPT)
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

            fn show_manager(&self) -> Result<()> {
                let manager = self.get_webview_window("manager").ok_or(anyhow!("Manager window not found"))?;
                manager.show()?;
                manager.set_focus()?;
                Ok(())
            }
        }
    };
}

shared_impl!(App<R>);
shared_impl!(AppHandle<R>);

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
