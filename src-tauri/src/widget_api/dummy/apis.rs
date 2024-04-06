use tauri::command;

#[command]
pub fn get_dummy_info() -> String {
    "Hello from Deskulpt dummy api!".to_string()
}

#[command]
pub fn shout_text(text: String) -> String {
    text.to_uppercase()
}
