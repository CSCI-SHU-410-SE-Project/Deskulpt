//! The module provides the types for state management in Tauri.
//!
//! In Tauri, different states are distinguished by their unique types, thus we
//! always use structs to mark the states.

use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::Mutex;

use tauri::menu::MenuItem;
use tauri::Runtime;

use crate::config::WidgetConfigCollection;

/// The type for the state of the collection of widget configurations.
///
/// The managed state will be updated at runtime and is thus protected by a
/// mutex.
#[derive(Default)]
pub(crate) struct WidgetConfigCollectionState(pub(crate) Mutex<WidgetConfigCollection>);

/// The type for the state of the widget base directory.
///
/// This contains the path to the base directory `$APPDATA/widgets/` where all
/// widgets should be locally stored. This state is static and should not be
/// changed during the runtime.
pub(crate) struct WidgetBaseDirectoryState(pub(crate) PathBuf);

impl WidgetBaseDirectoryState {
    /// Initialize the widget base directory state.
    ///
    /// This creates the widget base directory if it does not exist.
    pub(crate) fn init(base: PathBuf) -> Self {
        let widget_base_dir = base.join("widgets");
        if !widget_base_dir.exists() {
            create_dir_all(&widget_base_dir).unwrap();
        }
        Self(widget_base_dir)
    }
}

/// Canvas click-through state information.
pub(crate) struct CanvasClickThrough<R: Runtime> {
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
    pub(crate) fn toggle(&mut self) {
        self.yes = !self.yes;
        let _ = self
            .menu_item
            .set_text(if self.yes { "Float" } else { "Sink" });
    }

    /// Get whether the canvas is click-through.
    pub(crate) fn yes(&self) -> bool {
        self.yes
    }
}

/// The type for the state of whether the canvas can be clicked through.
///
/// The managed state will be updated at runtime and is thus protected by a
/// mutex.
pub(crate) struct CanvasClickThroughState<R: Runtime>(pub(crate) Mutex<CanvasClickThrough<R>>);

impl<R: Runtime> CanvasClickThroughState<R> {
    /// Initialize the canvas click-through state.
    pub(crate) fn init(is_click_through: bool, menu_item: MenuItem<R>) -> Self {
        Self(Mutex::new(CanvasClickThrough {
            yes: is_click_through,
            menu_item,
        }))
    }
}
