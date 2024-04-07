use tauri::command;

#[command]
pub async fn get_dummy_info(widget_id: String) -> String {
    format!("Hello from the dummy widget with ID: {}", widget_id)
}

#[command]
pub async fn shout_text(widget_id: String, text: String) -> String {
    format!("{}: {}", widget_id, text.to_uppercase())
}
