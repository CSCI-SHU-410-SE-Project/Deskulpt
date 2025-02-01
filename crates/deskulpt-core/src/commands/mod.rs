//! Deskulpt core commands to be invoked by the frontend.

#[doc(hidden)]
mod bundle_widget;
#[doc(hidden)]
mod call_plugin;
#[doc(hidden)]
mod exit_app;
#[doc(hidden)]
mod open_in_widgets_dir;
#[doc(hidden)]
mod rescan_widgets;
#[doc(hidden)]
mod update_toggle_shortcut;

mod error;

pub use bundle_widget::*;
pub use call_plugin::*;
pub use exit_app::*;
pub use open_in_widgets_dir::*;
pub use rescan_widgets::*;
pub use update_toggle_shortcut::*;
