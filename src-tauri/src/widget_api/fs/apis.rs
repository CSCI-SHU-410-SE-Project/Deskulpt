use crate::states::WidgetBaseDirectoryState;
use std::io::Write;
use std::path::PathBuf;
use tauri::{command, AppHandle, Manager, Runtime};

fn get_file_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> PathBuf {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;
    widget_base.join("widgets").join(widget_id).join("storage").join(path)
}

#[command]
pub fn read_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<String, String> {
    let file_path = get_file_path(&app_handle, &widget_id, &path);
    std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))
}

#[command]
pub fn write_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> Result<(), String> {
    let file_path = get_file_path(&app_handle, &widget_id, &path);
    std::fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write file '{}': {}", file_path.display(), e))
}

#[command]
pub fn append_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> Result<(), String> {
    let file_path = get_file_path(&app_handle, &widget_id, &path);
    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .map_err(|e| format!("Failed to append file '{}': {}", file_path.display(), e))
}

#[command]
pub fn remove_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<(), String> {
    let file_path = get_file_path(&app_handle, &widget_id, &path);
    std::fs::remove_file(&file_path)
        .map_err(|e| format!("Failed to delete file '{}': {}", file_path.display(), e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::test::{mock_app, MockRuntime};
    use tempfile::tempdir;

    fn setup_widget_environment(
        widget_id: &str,
    ) -> (AppHandle<MockRuntime>, tempfile::TempDir) {
        let temp_dir = tempdir().expect("Failed to create a temporary directory");
        let app_dir = temp_dir.path();

        let app = mock_app();
        let app_handle = app.handle();
        app_handle.manage(WidgetBaseDirectoryState(app_dir.to_path_buf()));

        let widget_dir = app_dir.join("widgets").join(widget_id).join("storage");
        std::fs::create_dir_all(&widget_dir)
            .expect("Failed to create widget storage directory");

        (app_handle, temp_dir)
    }

    #[test]
    fn test_write_and_read_file() {
        let widget_id = "test_widget_write_read";
        let (app_handle, _temp_dir) = setup_widget_environment(widget_id);
        let file_name = "test_file.txt";
        let content = "Hello, world!";

        // Test writing to the file
        assert!(write_file(
            app_handle.clone(),
            widget_id.to_string(),
            file_name.to_string(),
            content.to_string()
        )
        .is_ok());

        // Test reading the file
        let read_content =
            read_file(app_handle, widget_id.to_string(), file_name.to_string())
                .unwrap();
        assert_eq!(content, read_content);
    }

    #[test]
    fn test_append_and_remove_file() {
        let widget_id = "test_widget_append_remove";
        let (app_handle, _temp_dir) = setup_widget_environment(widget_id);
        let file_name = "append_file.txt";
        let content = "Hello, ";
        let append_content = "world!";

        // Write initial content
        write_file(
            app_handle.clone(),
            widget_id.to_string(),
            file_name.to_string(),
            content.to_string(),
        )
        .unwrap();

        // Append to the file
        assert!(append_file(
            app_handle.clone(),
            widget_id.to_string(),
            file_name.to_string(),
            append_content.to_string()
        )
        .is_ok());

        // Read and verify the content
        let read_content =
            read_file(app_handle.clone(), widget_id.to_string(), file_name.to_string())
                .unwrap();
        assert_eq!(format!("{}{}", content, append_content), read_content);

        // Test removing the file
        assert!(remove_file(app_handle, widget_id.to_string(), file_name.to_string())
            .is_ok());
    }
}
