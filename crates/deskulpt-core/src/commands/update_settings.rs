use serde::Deserialize;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::{ShortcutKey, Theme, WidgetSettings};
use crate::states::SettingsStateExt;

/// Message for updating settings.
#[derive(Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SettingsUpdate {
    /// Update the theme.
    Theme(Theme),
    /// Update a keyboard shortcut.
    ///
    /// The first element is the shortcut key, and the second element is the new
    /// shortcut value. `None` means to remove the shortcut.
    Shortcut(ShortcutKey, Option<String>),
    /// Update the settings of a widget.
    ///
    /// The first element is the widget ID, and the second element is the new
    /// widget settings.
    Widget(String, WidgetSettings),
}

/// Update the settings.
///
/// This command updates the settings state in the backend. If an update has
/// side effects, they will be applied prior to the update being committed.
///
/// ### Errors
///
/// - Failed to apply the side effects, if any.
#[command]
#[specta::specta]
pub async fn update_settings<R: Runtime>(
    app_handle: AppHandle<R>,
    update: SettingsUpdate,
) -> CmdResult<()> {
    match update {
        SettingsUpdate::Theme(theme) => {
            app_handle.update_settings_theme(theme);
        },
        SettingsUpdate::Shortcut(key, value) => {
            app_handle.update_settings_shortcut(key, value)?;
        },
        SettingsUpdate::Widget(id, settings) => {
            app_handle.update_settings_widget(id, settings);
        },
    }
    Ok(())
}
