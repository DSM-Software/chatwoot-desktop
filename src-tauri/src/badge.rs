use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn set_badge_count(app: AppHandle, count: u32) {
    if let Some(window) = app.get_webview_window("main") {
        let label = if count == 0 {
            None
        } else {
            Some(count.to_string())
        };
        let _ = window.set_badge_label(label);
    }
}
