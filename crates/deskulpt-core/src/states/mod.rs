//! Deskulpt runtime state management.

mod canvas_imode;
mod settings;
mod widgets;

#[doc(hidden)]
pub use canvas_imode::CanvasImodeStateExt;
#[doc(hidden)]
pub use settings::SettingsStateExt;
#[doc(hidden)]
pub use widgets::WidgetsStateExt;
