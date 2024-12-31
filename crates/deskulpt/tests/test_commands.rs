use deskulpt_test::commands::{bundle_widget, refresh_widget_collection};
use deskulpt_test_states::WidgetConfigMapState;
use deskulpt_test_testing::assert::assert_eq;
use deskulpt_test_testing::fixture_path;
use deskulpt_test_testing::mock::MockerBuilder;
use rstest::rstest;
use tauri::Manager;

#[rstest]
async fn test_refresh_widget_collection() {
    let mocker = MockerBuilder::default()
        .with_widgets_dir(fixture_path("deskulpt-config/widgets"))
        .build();

    // The command should not fail regardless of the contents of any widget
    let collection = refresh_widget_collection(mocker.handle().clone()).await;
    assert!(collection.is_ok());
    let collection = collection.unwrap();

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

    // Check the invalid configurations are recorded as errors; details should
    // be covered in deskulpt-config tests
    for name in invalid_configs {
        assert!(collection[name].is_err());
    }

    // Check that valid configurations are recorded with the correct directory;
    // details should be covered in deskulpt-config tests
    for name in valid_configs {
        assert!(collection[name].is_ok());
        assert_eq!(
            collection[name].as_ref().unwrap().directory,
            mocker.widgets_path(name),
        );
    }

    // Check that the widget collection state has been updated
    let state_collection = mocker.handle().state::<WidgetConfigMapState>();
    let state_collection = state_collection.0.lock().unwrap();
    assert_eq!(state_collection.clone(), collection);
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
