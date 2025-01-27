//! Deskulpt core commands to be invoked by the frontend.

#[doc(hidden)]
mod bundle_widget;
#[doc(hidden)]
mod call_plugin;
#[doc(hidden)]
mod emit_on_render_ready;
#[doc(hidden)]
mod exit_app;
#[doc(hidden)]
mod open_in_widgets_dir;
#[doc(hidden)]
mod rescan_widgets;
#[doc(hidden)]
mod set_render_ready;
#[doc(hidden)]
mod update_shortcuts;

mod error;

pub use bundle_widget::*;
pub use call_plugin::*;
pub use emit_on_render_ready::*;
pub use exit_app::*;
pub use open_in_widgets_dir::*;
pub use rescan_widgets::*;
pub use set_render_ready::*;
pub use update_shortcuts::*;
