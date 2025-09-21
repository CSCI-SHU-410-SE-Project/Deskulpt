use serde::Deserialize;
use tauri::{command, AppHandle, Runtime};
use tauri_specta::Event;

use super::error::CmdResult;
use crate::events::UpdateSettingsEvent;
use crate::settings::{ShortcutKey, Theme};
use crate::states::SettingsStateExt;

/// Message for updating widget settings.
#[derive(Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSettingsUpdate {
    /// [`WidgetSettings::x`](crate::settings::WidgetSettings::x)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = i32)]
    x: Option<i32>,
    /// [`WidgetSettings::y`](crate::settings::WidgetSettings::y)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = i32)]
    y: Option<i32>,
    /// [`WidgetSettings::opacity`](crate::settings::WidgetSettings::opacity)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = i32)]
    opacity: Option<i32>,
}

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
    /// widget settings. If the widget ID does not exist, this is an error.
    Widget(String, WidgetSettingsUpdate),
}

/// Update the settings.
///
/// This command updates the settings state in the backend. If an update has
/// side effects, they will be applied prior to the update being committed. See
/// [`SettingsStateExt`] for more information.
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
        SettingsUpdate::Widget(id, update) => {
            app_handle.update_settings_widget(id, update.x, update.y, update.opacity)?;
        },
    }

    let settings = app_handle.get_settings();
    UpdateSettingsEvent(settings.clone()).emit(&app_handle)?;
    Ok(())
}
