#[tauri::command]
pub fn custom_command(message: &str) -> String {
    println!("From JS: {message}");
    format!("{message}")
}
