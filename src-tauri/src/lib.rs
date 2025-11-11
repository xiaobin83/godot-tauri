use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref IsConsoleOpened : Mutex<bool> = Mutex::new(false);
}

#[tauri::command]
fn toggle_console(window: tauri::WebviewWindow) {
    let mut is_console_opened= IsConsoleOpened.lock().unwrap();
    *is_console_opened = !*is_console_opened;
    if *is_console_opened {
        window.open_devtools();
    } else {
        window.close_devtools();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![toggle_console])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
