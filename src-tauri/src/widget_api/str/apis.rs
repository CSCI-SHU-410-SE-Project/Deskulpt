use tauri::command;

#[command]
pub async fn get_text(widget_id: String) -> String {
    format!("Hello from the dummy widget with ID: {}", widget_id)
}
