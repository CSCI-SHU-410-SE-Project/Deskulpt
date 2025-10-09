//! Deskulpt runtime state management.

mod canvas_imode;
mod initial_render;
mod package_manager;
mod settings;
mod widget_config_map;

#[doc(hidden)]
pub use canvas_imode::CanvasImodeStateExt;
#[doc(hidden)]
pub use initial_render::InitialRenderStateExt;
#[doc(hidden)]
pub use package_manager::PackageManagerStateExt;
pub use package_manager::{PackageManagerDetection, PackageManagerInfo, PackageManagerKind};
#[doc(hidden)]
pub use settings::SettingsStateExt;
#[doc(hidden)]
pub use widget_config_map::WidgetConfigMapStateExt;
