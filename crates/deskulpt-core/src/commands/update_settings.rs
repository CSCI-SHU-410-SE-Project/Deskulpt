use serde::Deserialize;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::{ShortcutKey, Theme, WidgetSettings};
use crate::states::SettingsStateExt;

#[derive(Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SettingsUpdate {
    Theme(Theme),
    Shortcut(ShortcutKey, Option<String>),
    Widget(String, WidgetSettings),
}

/// TODO(Charlie-XIAO)
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
