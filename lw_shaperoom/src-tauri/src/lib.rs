use tauri::{AppHandle, Emitter}; 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![cmd_send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn log_to_clog(app: &AppHandle, message: String) {
    // This sends a 'c-log' event to all windows
    app.emit("c-log", message).unwrap();
}
/// message to emit
#[tauri::command]
fn cmd_send_message(app: AppHandle, message: String) {
    log_to_clog(&app, message)
}