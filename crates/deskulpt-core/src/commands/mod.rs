//! Deskulpt core commands to be invoked by the frontend.

#[doc(hidden)]
mod bundle_widgets;
#[doc(hidden)]
mod call_plugin;
#[doc(hidden)]
mod load_widgets;
#[doc(hidden)]
mod mark_setup;
#[doc(hidden)]
mod open_widget;
#[doc(hidden)]
mod update_settings;

mod error;

pub use bundle_widgets::*;
pub use call_plugin::*;
pub use load_widgets::*;
pub use mark_setup::*;
pub use open_widget::*;
pub use update_settings::*;
