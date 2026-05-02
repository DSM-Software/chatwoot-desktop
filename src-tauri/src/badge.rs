use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn set_badge_count(app: AppHandle, count: u32) {
    // set_badge_label is macOS-only; Windows taskbar badges require a
    // different API (overlay icon) not yet implemented.
    #[cfg(target_os = "macos")]
    if let Some(window) = app.get_webview_window("main") {
        let label = if count == 0 {
            None
        } else {
            Some(count.to_string())
        };
        let _ = window.set_badge_label(label);
    }

    #[cfg(not(target_os = "macos"))]
    let _ = (app, count);
}
