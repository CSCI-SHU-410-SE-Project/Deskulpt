#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use anyhow::Error;
use deskulpt_test_config::WidgetCollection;
use deskulpt_test_events::{EventsExt, ShowToastPayload};
use tauri::{App, AppHandle, Manager, Runtime};

/// Managed state for the widget collection.
#[derive(Default)]
struct WidgetCollectionState(Mutex<WidgetCollection>);

/// Managed state for the widgets directory.
struct WidgetsDirState(PathBuf);

/// Managed state for whether the canvas is click-through.
struct CanvasClickThroughState(AtomicBool);

/// Extension trait for state management in Deskulpt.
pub trait StatesExt<R: Runtime>: Manager<R> + EventsExt<R> {
    /// Initialize state management for the widget collection.
    fn manage_widget_collection(&self) {
        self.manage(WidgetCollectionState::default());
    }

    /// Initialize state management for the widgets directory.
    ///
    /// This will create the widgets directory if it does not exist.
    fn manage_widgets_dir(&self) {
        let resource_dir = self.path().resource_dir().unwrap();
        let widgets_dir = resource_dir.join("widgets");
        if !widgets_dir.exists() {
            create_dir_all(&widgets_dir).unwrap();
        }
        self.manage(WidgetsDirState(widgets_dir));
    }

    /// Initialize state management for the widgets directory at a custom path.
    ///
    /// This is intended for testing purposes.
    #[cfg(feature = "testing")]
    fn manage_widgets_dir_at<P: AsRef<Path>>(&self, widgets_dir: P) {
        self.manage(WidgetsDirState(widgets_dir.as_ref().to_path_buf()));
    }

    /// Initialize state management for whether the canvas is click-through.
    ///
    /// The canvas is click-through by default.
    fn manage_canvas_click_through(&self) {
        self.manage(CanvasClickThroughState(AtomicBool::new(true)));
    }

    /// Provide reference to the widget collection for a closure.
    ///
    /// This will lock the widget collection state. The return value of the
    /// closure will be propagated.
    fn with_widget_collection<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&WidgetCollection) -> T,
    {
        let state = self.state::<WidgetCollectionState>();
        let widget_collection = state.0.lock().unwrap();
        f(&widget_collection)
    }

    /// Provide mutable reference to the widget collection for a closure.
    ///
    /// This will lock the widget collection state. The return value of the
    /// closure will be propagated.
    fn with_widget_collection_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut WidgetCollection) -> T,
    {
        let state = self.state::<WidgetCollectionState>();
        let mut widget_collection = state.0.lock().unwrap();
        f(&mut widget_collection)
    }

    /// Get the widgets directory.
    ///
    /// This involves a cheap cloning operation.
    fn widgets_dir(&self) -> PathBuf {
        self.state::<WidgetsDirState>().0.clone()
    }

    /// Toggle the click-through state of the canvas window.
    fn toggle_canvas_click_through(&self) -> Result<(), Error> {
        let canvas = self
            .get_webview_window("canvas")
            .expect("Canvas window not found");

        let state = self.state::<CanvasClickThroughState>();
        let prev_click_through = state.0.load(Ordering::Relaxed);
        canvas.set_ignore_cursor_events(!prev_click_through)?;
        state.0.store(!prev_click_through, Ordering::Relaxed);

        // If the canvas is toggled to not click-through, try to regain focus to
        // avoid flickering on the first click; consume any error because this
        // is not critical
        if prev_click_through {
            let _ = canvas.set_focus();
        }

        // Show a toast message on the canvas
        let message = if prev_click_through {
            "Canvas floated."
        } else {
            "Canvas sunk."
        };
        let _ = self.emit_show_toast_to_canvas(ShowToastPayload::Success(message.to_string()));
        Ok(())
    }
}

impl<R: Runtime> StatesExt<R> for AppHandle<R> {}

impl<R: Runtime> StatesExt<R> for App<R> {}
