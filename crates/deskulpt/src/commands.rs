//! Deskulpt core commands.

use std::collections::HashMap;
use std::fs::read_dir;
use std::path::PathBuf;

use anyhow::Context;
use deskulpt_test_bundler::WidgetBundler;
use deskulpt_test_config::{WidgetCollection, WidgetConfig};
use deskulpt_test_settings::GlobalSetting;
use deskulpt_test_states::StatesExt;
use deskulpt_test_utils::{cmdbail, cmderr, CommandOut};
use tauri::{command, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tauri_plugin_opener::OpenerExt;

/// Command for refreshing the state of the widget collection.
///
/// This command will scan through the widget base directory and update the
/// current widget collection state with the new widgets found. It will also
/// return the updated widget collection, intended to be used by the frontend to
/// refresh the rendering of the widgets.
///
/// This command will fail if:
///
/// - There is an error reading the widget base directory.
/// - There is an error getting some entry in the widget base directory.
/// - There is an error inferring the widget ID from the path of the entry.
///
/// Note that failure to load a widget configuration will not lead to an overall
/// failure of the command. Instead, the widget ID will correspond to an error
/// message instead of a widget configuration.
#[command]
pub async fn refresh_widget_collection<R: Runtime>(
    app_handle: AppHandle<R>,
) -> CommandOut<WidgetCollection> {
    let widgets_dir = app_handle.widgets_dir();
    let mut new_widget_collection = HashMap::new();

    let entries = match read_dir(widgets_dir) {
        Ok(entries) => entries,
        Err(e) => cmdbail!(e),
    };

    for entry in entries {
        // There could be intermittent IO errors during iteration
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => cmdbail!(e),
        };

        let path = entry.path();
        if !path.is_dir() {
            continue; // Non-directory entries are not widgets, skip
        }
        let widget_id = match path.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => cmdbail!("Cannot infer widget ID from '{}'", path.display()),
        };

        // Load the widget configuration and raise on error
        let widget_config = match WidgetConfig::try_read(&path) {
            Ok(widget_config) => widget_config,
            Err(e) => {
                // We should not fail the whole command if some widget configuration
                // fails to be loaded; instead we record the error corresponding to the
                // widget ID
                new_widget_collection.insert(widget_id, Err(cmderr!(e)));
                continue;
            },
        };

        // Widget configuration being `None` means that the directory is not a widget
        // that is meant to be rendered
        if let Some(widget_config) = widget_config {
            new_widget_collection.insert(widget_id, Ok(widget_config));
        }
    }

    // Update the widget collection state
    app_handle.with_widget_collection_mut(|collection| {
        collection.clone_from(&new_widget_collection);
    });
    Ok(new_widget_collection)
}

/// Command for bundling the specified widget.
///
/// The widget configuration will be obtained by searching the managed widget
/// collection for the given widget ID. The widget will be bundled into a string
/// of ESM code if the ID is found in the collection.
///
/// The command also requires the URL of the APIs blob of the widget. This is
/// used for replacing the imports of `@deskulpt-test/apis` by the actual URL to
/// import from.
///
/// This command will fail if:
///
/// - The widget ID is not found in the state of the widget collection.
/// - The widget collection state corresponding to the widget ID is an error
///   message instead of a widget configuration.
/// - There is an error when bundling the widget.
#[command]
pub async fn bundle_widget<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    apis_blob_url: String,
) -> CommandOut<String> {
    let bundler = app_handle.with_widget_collection(|collection| {
        if let Some(config) = collection.get(&widget_id) {
            match config.as_ref() {
                Ok(config) => Ok(WidgetBundler::new(
                    config.directory().to_path_buf(),
                    config.entry_path(),
                    apis_blob_url,
                    config.external_deps(),
                )),
                // Propagate widget configuration error
                Err(e) => cmdbail!(e.clone()),
            }
        } else {
            cmdbail!("Widget '{widget_id}' is not found in the collection")
        }
    })?;

    bundler
        .bundle()
        .context(format!("Failed to bundle widget (id={})", widget_id))
        .map_err(|e| cmderr!(e))
}

/// (Un)register a global shortcut for toggling the click-through state.
///
/// If `reverse` is `false` this will register the shortcut, otherwise it will
/// unregister it.
///
/// This command will fail if:
///
/// - The shortcut is already registered but we are registering it again.
/// - The shortcut is not registered yet but we want to unregister it.
/// - There is an error registering or unregistering the shortcut.
#[command]
pub async fn register_toggle_shortcut<R: Runtime>(
    app_handle: AppHandle<R>,
    shortcut: String,
    reverse: bool,
) -> CommandOut<()> {
    let manager = app_handle.global_shortcut();
    let shortcut = shortcut.as_str();

    if reverse {
        // We want to unregister
        if !manager.is_registered(shortcut) {
            cmdbail!("'{shortcut}' is not registered and cannot be unregistered");
        }
        manager.unregister(shortcut).map_err(|e| cmderr!(e))
    } else {
        // We want to register
        if manager.is_registered(shortcut) {
            cmdbail!("'{shortcut}' is registered and cannot be registered again");
        }
        manager
            .on_shortcut(shortcut, |app_handle, _, event| {
                if event.state == ShortcutState::Pressed {
                    // We must only react to press events, otherwise we would toggle
                    // back on release
                    let _ = app_handle.toggle_canvas_click_through();
                }
            })
            .map_err(|e| cmderr!(e))
    }
}

/// Command for opening a widget-related resource.
///
/// If widget ID is `None`, this command will open the widget base directory.
/// Otherwise, it checks whether the widget ID. If `path` is `None`, it opens
/// the corresponding widget directory; otherwise it opens the specified path
/// within the widget directory.
///
///
/// This command will fail if:
///
/// - The given widget ID is not found in the widget collection.
/// - Tauri fails to open the resource.
#[command]
pub async fn open_widget_resource<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: Option<String>,
    path: Option<PathBuf>,
) -> CommandOut<()> {
    let widgets_dir = app_handle.widgets_dir();

    let open_path = match widget_id {
        Some(widget_id) => app_handle.with_widget_collection(|collection| {
            if !collection.contains_key(&widget_id) {
                cmdbail!("Widget '{}' is not found in the collection", widget_id)
            }
            let widget_dir = widgets_dir.join(widget_id);
            match path {
                Some(path) => Ok(widget_dir.join(path)),
                None => Ok(widget_dir),
            }
        })?,
        None => widgets_dir.to_path_buf(),
    };

    app_handle
        .opener()
        .open_path(open_path.to_string_lossy(), None::<&str>)
        .map_err(|e| cmderr!(e))
}

/// Command for initializing the global settings.
///
/// This command tries to load the previously stored settings. It never fails,
/// but instead returns the default settings upon any error.
#[command]
pub async fn init_global_setting<R: Runtime>(
    app_handle: AppHandle<R>,
) -> CommandOut<GlobalSetting> {
    let app_data_dir = match app_handle.path().app_data_dir() {
        Ok(app_data_dir) => app_data_dir,
        Err(_) => return Ok(Default::default()),
    };
    Ok(GlobalSetting::read(&app_data_dir))
}

/// Command for cleaning up and exiting the application.
///
/// This command will try to save the widget internals for persistence before
/// exiting the application, but failure to do so will not prevent the
/// application from exiting.
#[command]
pub async fn exit_app<R: Runtime>(
    app_handle: AppHandle<R>,
    global_setting: GlobalSetting,
) -> CommandOut<()> {
    let app_data_dir = match app_handle.path().app_data_dir() {
        Ok(app_data_dir) => app_data_dir,
        Err(_) => {
            app_handle.exit(0);
            return Ok(());
        },
    };

    let _ = global_setting.try_write(app_data_dir);
    app_handle.exit(0);
    Ok(())
}

#[cfg(test)]
mod tests {
    use deskulpt_test_testing::fixture_path;
    use deskulpt_test_testing::mock::MockerBuilder;
    use rstest::rstest;

    use super::*;

    #[rstest]
    async fn test_refresh_widget_collection() {
        let mocker = MockerBuilder::default()
            .with_widgets_dir(fixture_path("deskulpt-config/widgets"))
            .build();

        // The command should not fail regardless of the contents of any widget
        let collection = refresh_widget_collection(mocker.handle().clone()).await;
        assert!(collection.is_ok());
        let collection = collection.unwrap();

        // Check that the widget collection state is consistent with the
        // returned collection
        mocker
            .handle()
            .with_widget_collection(|collection_in_state| {
                assert_eq!(collection_in_state.clone(), collection);
            });

        // Check that we have got the expected number of widgets
        let invalid_configs = [
            "conf_missing_field",
            "conf_not_readable",
            "package_json_not_readable",
        ];
        let valid_configs = ["all_fields", "package_json_no_deps", "package_json_none"];
        assert_eq!(
            collection.len(),
            invalid_configs.len() + valid_configs.len()
        );

        // Check that configurations are correctly recorded as Ok or Err; details
        // should be covered in deskulpt-config tests
        for name in invalid_configs {
            assert!(collection[name].is_err());
        }
        for name in valid_configs {
            assert!(collection[name].is_ok());
        }
    }

    #[rstest]
    async fn test_bundle_widget() {
        let mocker = MockerBuilder::default()
            .with_widgets_dir(fixture_path("deskulpt-config/widgets"))
            .build();
        let collection = refresh_widget_collection(mocker.handle().clone())
            .await
            .unwrap();

        // Check that error is raised for non-existent widget ID
        let result = bundle_widget(
            mocker.handle().clone(),
            "non_existent".to_string(),
            Default::default(),
        )
        .await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Widget 'non_existent' is not found in the collection"
        );

        // Check that error message for invalid configuration gets propagated
        let result = bundle_widget(
            mocker.handle().clone(),
            "conf_missing_field".to_string(),
            Default::default(),
        )
        .await;
        let err_msg = collection
            .get("conf_missing_field")
            .unwrap()
            .as_ref()
            .unwrap_err();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), err_msg.clone());

        // Check that bundling error is raised (in this case we have bundling error
        // because the entry point file is missing)
        let result = bundle_widget(
            mocker.handle().clone(),
            "all_fields".to_string(),
            Default::default(),
        )
        .await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to bundle widget (id=all_fields)"));

        // Create the entry point file and expect Ok result; details should be
        // covered in deskulpt-bundler tests
        std::fs::write(mocker.widgets_path("all_fields/index.jsx"), "").unwrap();
        let result = bundle_widget(
            mocker.handle().clone(),
            "all_fields".to_string(),
            Default::default(),
        )
        .await;
        assert!(result.is_ok());
    }
}
