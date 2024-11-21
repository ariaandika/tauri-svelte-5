// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod command;
mod channel;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            command::custom_command,
            channel::progress_channel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
