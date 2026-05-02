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
    #[cfg(target_os = "macos")]
    {
        use tauri_plugin_notification::PermissionState;

        let permission = app
            .notification()
            .permission_state()
            .map_err(|e| format!("Failed to get permission state: {}", e))?;

        return match permission {
            PermissionState::Granted => Ok("granted".to_string()),
            PermissionState::Denied => Ok("denied".to_string()),
            _ => {
                let result = app
                    .notification()
                    .request_permission()
                    .map_err(|e| format!("Failed to request permission: {}", e))?;

                match result {
                    PermissionState::Granted => Ok("granted".to_string()),
                    _ => Ok("denied".to_string()),
                }
            }
        };
    }

    // On Windows and other platforms, notification access is managed via OS settings.
    // No runtime permission dialog is available.
    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
        return Ok("granted".to_string());
    }
}
