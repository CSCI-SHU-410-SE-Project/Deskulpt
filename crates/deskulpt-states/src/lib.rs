//! The module provides the types for state management in Tauri.
//!
//! In Tauri, different states are distinguished by their unique types, thus we
//! always use structs to mark the states.

#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::{bail, Error};
use deskulpt_test_config::WidgetConfigMap;
use serde::Serialize;
use tauri::menu::MenuItem;
use tauri::{AppHandle, Emitter, Manager, Runtime};

/// The type for the state of the collection of widget configurations.
///
/// The managed state will be updated at runtime and is thus protected by a
/// mutex.
#[derive(Default)]
pub struct WidgetConfigMapState(pub Mutex<WidgetConfigMap>);

/// The type for the state of the widget base directory.
///
/// This contains the path to the base directory `$APPDATA/widgets/` where all
/// widgets should be locally stored. This state is static and should not be
/// changed during the runtime.
pub struct WidgetBaseDirectoryState(pub PathBuf);

impl WidgetBaseDirectoryState {
    /// Initialize the widget base directory state.
    ///
    /// This creates the widget base directory if it does not exist.
    pub fn init(base: PathBuf) -> Self {
        let widget_base_dir = base.join("widgets");
        if !widget_base_dir.exists() {
            create_dir_all(&widget_base_dir).unwrap();
        }
        Self(widget_base_dir)
    }
}

/// Canvas click-through state information.
pub struct CanvasClickThrough<R: Runtime> {
    /// Whether the canvas is click-through.
    yes: bool,
    /// The menu item for toggling the canvas click-through state.
    menu_item: MenuItem<R>,
}

impl<R: Runtime> CanvasClickThrough<R> {
    /// Try to toggle the canvas click-through state.
    ///
    /// This is guaranteed to update whether the canvas is click-through or not.
    /// It may, however, fail to update the menu item text without an error
    /// beccause it is not worth panicking for such a minor thing.
    pub fn toggle(&mut self) {
        self.yes = !self.yes;
        let _ = self
            .menu_item
            .set_text(if self.yes { "Float" } else { "Sink" });
    }

    /// Get whether the canvas is click-through.
    pub fn yes(&self) -> bool {
        self.yes
    }
}

/// The type for the state of whether the canvas can be clicked through.
///
/// The managed state will be updated at runtime and is thus protected by a
/// mutex.
pub struct CanvasClickThroughState<R: Runtime>(pub Mutex<CanvasClickThrough<R>>);

impl<R: Runtime> CanvasClickThroughState<R> {
    /// Initialize the canvas click-through state.
    pub fn init(is_click_through: bool, menu_item: MenuItem<R>) -> Self {
        Self(Mutex::new(CanvasClickThrough {
            yes: is_click_through,
            menu_item,
        }))
    }
}

/// Toast kind of the "show-toast" event.
#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum ToastKind {
    Success,
}

/// Payload of the "show-toast" event.
#[derive(Serialize, Clone)]
struct ShowToastPayload {
    kind: ToastKind,
    message: String,
}

/// Toggle the click-through state of the canvas window.
///
/// This will toggle whether the canvas window ignores cursor events and update
/// the state accordingly. If the canvas is toggled to not click-through, it
/// will try to regain focus automatically. The function will fail if:
///
/// - The canvas window is not found.
/// - Fails to set the canvas to ignore/unignore cursor events.
pub fn toggle_click_through_state<R: Runtime>(app_handle: &AppHandle<R>) -> Result<(), Error> {
    let canvas = match app_handle.get_webview_window("canvas") {
        Some(canvas) => canvas,
        None => bail!("Canvas window not found"),
    };

    let click_through_state = &app_handle.state::<CanvasClickThroughState<R>>();
    let mut click_through = click_through_state.0.lock().unwrap();
    let prev_can_click_through = click_through.yes();

    // Try to toggle the click through state
    canvas.set_ignore_cursor_events(!prev_can_click_through)?;
    click_through.toggle();

    // If the canvas is previously click-through, meaning that it is now set to not
    // click-through, try to regain focus to avoid flickering on the first click
    if prev_can_click_through {
        let _ = canvas.set_focus(); // Consume any error because this is not
                                    // critical
    }

    // Try to let canvas show the toast message
    let _ = app_handle.emit_to(
        "canvas",
        "show-toast",
        ShowToastPayload {
            kind: ToastKind::Success,
            message: format!(
                "Canvas {}.",
                if prev_can_click_through {
                    "floated"
                } else {
                    "sunk"
                }
            ),
        },
    );
    Ok(())
}
