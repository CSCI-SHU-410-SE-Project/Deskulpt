//! The module provides the commands used internally by Deskulpt.

use std::collections::HashMap;
use std::fs::read_dir;
use std::path::PathBuf;

use anyhow::Context;
use deskulpt_test_bundler::bundle;
use deskulpt_test_config::{read_widget_config, WidgetConfigMap};
use deskulpt_test_settings::{read_settings, write_settings, Settings};
use deskulpt_test_states::{
    toggle_click_through_state, WidgetBaseDirectoryState, WidgetConfigMapState,
};
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
) -> CommandOut<WidgetConfigMap> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;
    let mut new_widget_collection = HashMap::new();

    let entries = match read_dir(widget_base) {
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
        let widget_config = match read_widget_config(&path) {
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
    let widget_collection = app_handle.state::<WidgetConfigMapState>();
    widget_collection
        .0
        .lock()
        .unwrap()
        .clone_from(&new_widget_collection);
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
    let widget_collection_state = &app_handle.state::<WidgetConfigMapState>();
    let widget_collection = widget_collection_state.0.lock().unwrap();

    if let Some(widget_config) = widget_collection.get(&widget_id) {
        let widget_config = match widget_config.as_ref() {
            Ok(widget_config) => widget_config,
            Err(e) => cmdbail!(e.clone()),
        };
        // Obtain the absolute path of the widget entry point
        let widget_entry = &widget_config
            .directory
            .join(&widget_config.deskulpt_conf.entry);

        // Wrap the bundled code if success, otherwise let the error propagate
        return bundle(
            &widget_config.directory,
            widget_entry,
            apis_blob_url,
            &widget_config.external_deps,
        )
        .context(format!("Failed to bundle widget (id={})", widget_id))
        .map_err(|e| cmderr!(e));
    }

    // Error out if the widget ID is not found in the collection
    cmdbail!("Widget '{widget_id}' is not found in the collection")
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
            .on_shortcut(shortcut, |handle, _, event| {
                if event.state == ShortcutState::Pressed {
                    // We must only react to press events, otherwise we would toggle
                    // again on release; also consume errors because they are not
                    // allowed to propagate
                    let _ = toggle_click_through_state(handle);
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
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;

    let open_path = match widget_id {
        Some(widget_id) => {
            let widget_collection = app_handle.state::<WidgetConfigMapState>();
            if !widget_collection.0.lock().unwrap().contains_key(&widget_id) {
                cmdbail!("Widget '{}' is not found in the collection", widget_id)
            }
            let widget_dir = widget_base.join(widget_id);
            match path {
                Some(path) => widget_dir.join(path),
                None => widget_dir,
            }
        },
        None => widget_base.to_path_buf(),
    };

    app_handle
        .opener()
        .open_path(open_path.to_string_lossy(), None::<&str>)
        .map_err(|e| cmderr!(e))
}

/// Command for initializing the settings.
///
/// This command tries to load the previously stored settings. It never fails,
/// but instead returns the default settings upon any error.
#[command]
pub async fn init_settings<R: Runtime>(app_handle: AppHandle<R>) -> CommandOut<Settings> {
    let app_config_dir = match app_handle.path().app_config_dir() {
        Ok(app_config_dir) => app_config_dir,
        Err(_) => return Ok(Default::default()),
    };
    Ok(read_settings(&app_config_dir))
}

/// Command for cleaning up and exiting the application.
///
/// This command will try to save the widget internals for persistence before
/// exiting the application, but failure to do so will not prevent the
/// application from exiting.
#[command]
pub async fn exit_app<R: Runtime>(app_handle: AppHandle<R>, settings: Settings) -> CommandOut<()> {
    let app_config_dir = match app_handle.path().app_config_dir() {
        Ok(app_config_dir) => app_config_dir,
        Err(_) => {
            app_handle.exit(0);
            return Ok(());
        },
    };

    let _ = write_settings(&app_config_dir, &settings);
    app_handle.exit(0);
    Ok(())
}

#[cfg(test)]
mod tests {
    use copy_dir::copy_dir;
    use deskulpt_test_config::{DeskulptConf, WidgetConfig};
    use deskulpt_test_testing::assert::assert_eq;
    use deskulpt_test_testing::fixture_path;
    use deskulpt_test_testing::mock::setup_mock_env;
    use rstest::{fixture, rstest};
    use tauri::test::MockRuntime;
    use tempfile::TempDir;

    use super::*;

    /// Set up the environment for the `bundle_widget` command tests.
    #[fixture]
    #[once]
    fn setup_bundle_widget_env() -> (TempDir, AppHandle<MockRuntime>) {
        let (base_dir, app_handle) = setup_mock_env();
        {
            let widget_collection = app_handle.state::<WidgetConfigMapState>();
            let mut widget_collection = widget_collection.0.lock().unwrap();

            let dummy_deskulpt_conf = DeskulptConf {
                entry: "index.jsx".to_string(),
                name: "dummy".to_string(),
                ignore: false,
            };

            // Prepare a valid widget configuration and make the simplistic entry file
            let pass_widget_dir = base_dir.path().join("widgets/pass");
            widget_collection.insert(
                "pass".to_string(),
                Ok(WidgetConfig {
                    directory: pass_widget_dir.clone(),
                    deskulpt_conf: dummy_deskulpt_conf.clone(),
                    external_deps: Default::default(),
                }),
            );
            std::fs::create_dir_all(&pass_widget_dir).unwrap();
            std::fs::write(pass_widget_dir.join("index.jsx"), "").unwrap();

            // Prepare a valid widget configuration, but trigger a bundling error by
            // simply not creating the entry file; this is sufficient for the bundling
            // error case and details should be checked in the bundler unit tests
            let fail_widget_dir = base_dir.path().join("widgets/fail");
            widget_collection.insert(
                "fail".to_string(),
                Ok(WidgetConfig {
                    directory: fail_widget_dir.clone(),
                    deskulpt_conf: dummy_deskulpt_conf.clone(),
                    external_deps: Default::default(),
                }),
            );

            // Prepare an invalid widget configuration
            widget_collection.insert(
                "invalid_conf".to_string(),
                Err("Invalid configuration message".to_string()),
            );
        }

        (base_dir, app_handle)
    }

    #[rstest]
    async fn test_refresh_widget_collection() {
        // Test the `refresh_widget_collection` command
        let (base_dir, app_handle) = setup_mock_env();

        // Copy all configuration fixtures to the widget base directory; note that
        // this command does not care about the actual widget source code but only
        // configurations, so this would be enough for it to work
        let widget_base = base_dir.path().join("widgets").to_path_buf();
        println!("source: {:?}", fixture_path("config"));
        copy_dir(fixture_path("config"), &widget_base).unwrap();

        // The command should not fail just because contents of any configuration file
        let new_collection = refresh_widget_collection(app_handle.clone()).await;
        assert!(new_collection.is_ok());
        let new_collection = new_collection.unwrap();

        // Check that we have got all the expected configurations
        let invalid_configurations = [
            "conf_missing_field",
            "conf_not_readable",
            "package_json_not_readable",
        ];
        let valid_configurations = [
            "standard",
            "no_package_json",
            "package_json_no_dependencies",
        ];
        assert_eq!(
            new_collection.len(),
            invalid_configurations.len() + valid_configurations.len(),
            "The refreshed widget collection is missing some configurations",
        );

        // Invalid configurations should be recorded as errors; details should be
        // checked in configuration unit tests and error stringification is tested
        // separately
        for name in invalid_configurations {
            assert!(new_collection[name].is_err());
        }

        // Valid configurations; we only check the directory and others should be
        // checked in configuration unit tests
        for name in valid_configurations {
            assert!(new_collection[name].is_ok());
            assert_eq!(
                new_collection[name].as_ref().unwrap().directory,
                widget_base.join(name),
            );
        }

        // Check that the widget collection state has been updated
        let widget_collection = app_handle.state::<WidgetConfigMapState>();
        let widget_collection = widget_collection.0.lock().unwrap();
        assert_eq!(widget_collection.clone(), new_collection);
    }

    #[rstest]
    async fn test_bundle_widget_pass(setup_bundle_widget_env: &(TempDir, AppHandle<MockRuntime>)) {
        // Test that the `bundle_widget` command bundles a widget correctly
        let (_base_dir, app_handle) = setup_bundle_widget_env;
        let result =
            bundle_widget(app_handle.clone(), "pass".to_string(), Default::default()).await;

        // We only check that the result is Ok; the actual bundled content should be
        // checked in the bundler unit tests
        assert!(result.is_ok());
    }

    #[rstest]
    async fn test_bundle_widget_bundling_error(
        setup_bundle_widget_env: &(TempDir, AppHandle<MockRuntime>),
    ) {
        // Test that the `bundle_widget` command raises upon bundling error
        let (_base_dir, app_handle) = setup_bundle_widget_env;
        let result =
            bundle_widget(app_handle.clone(), "fail".to_string(), Default::default()).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(
            error.contains("Failed to bundle widget (id=fail)"),
            "The error message is not as expected: '{error}'",
        );
    }

    #[rstest]
    async fn test_bundle_widget_id_not_found(
        setup_bundle_widget_env: &(TempDir, AppHandle<MockRuntime>),
    ) {
        // Test that the `bundle_widget` command raises for an unknown widget ID
        let (_base_dir, app_handle) = setup_bundle_widget_env;
        let result = bundle_widget(
            app_handle.clone(),
            "non_existent_id".to_string(),
            Default::default(),
        )
        .await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error,
            "Widget 'non_existent_id' is not found in the collection"
        );
    }

    #[rstest]
    async fn test_bundle_widget_invalid_conf(
        setup_bundle_widget_env: &(TempDir, AppHandle<MockRuntime>),
    ) {
        // Test that the `bundle_widget` command propagates the error message held in
        // an invalid widget configuration
        let (_base_dir, app_handle) = setup_bundle_widget_env;
        let result = bundle_widget(
            app_handle.clone(),
            "invalid_conf".to_string(),
            Default::default(),
        )
        .await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, "Invalid configuration message");
    }
}
