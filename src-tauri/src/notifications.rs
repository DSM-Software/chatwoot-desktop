use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
pub fn send_notification(app: AppHandle, title: String, body: String) -> Result<(), String> {
    app.notification()
        .builder()
        .title(&title)
        .body(&body)
        .show()
        .map_err(|e| format!("Failed to send notification: {}", e))
}

#[tauri::command]
pub fn request_notification_permission(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_notification::PermissionState;

    let permission = app
        .notification()
        .permission_state()
        .map_err(|e| format!("Failed to get permission state: {}", e))?;

    match permission {
        PermissionState::Granted => Ok("granted".to_string()),
        PermissionState::Denied => Ok("denied".to_string()),
        _ => {
            // Request permission
            let result = app
                .notification()
                .request_permission()
                .map_err(|e| format!("Failed to request permission: {}", e))?;

            match result {
                PermissionState::Granted => Ok("granted".to_string()),
                _ => Ok("denied".to_string()),
            }
        }
    }
}
