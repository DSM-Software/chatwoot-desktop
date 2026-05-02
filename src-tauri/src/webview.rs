use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager};

/// Shared workspace URL for navigation interception (set at window creation).
static WORKSPACE_URL: std::sync::OnceLock<Arc<Mutex<String>>> = std::sync::OnceLock::new();

pub fn workspace_url_state() -> &'static Arc<Mutex<String>> {
    WORKSPACE_URL.get_or_init(|| Arc::new(Mutex::new(String::new())))
}

pub fn update_workspace_url(url: &str) {
    if let Ok(mut guard) = workspace_url_state().lock() {
        *guard = url.to_string();
    }
}

/// Navigate the main WebView to the given workspace URL.
pub fn load_workspace(app: &AppHandle, url: &str) -> Result<(), String> {
    update_workspace_url(url);

    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    let parsed: url::Url = url
        .parse()
        .map_err(|e| format!("Invalid URL: {}", e))?;

    window
        .navigate(parsed)
        .map_err(|e| format!("Navigation failed: {}", e))?;

    Ok(())
}

/// Bridge scripts injected into the workspace WebView after load.
fn notification_bridge_js() -> &'static str {
    include_str!("../../src/js/notification-bridge.js")
}

fn badge_bridge_js() -> &'static str {
    include_str!("../../src/js/badge-bridge.js")
}

/// Called by on_page_load in lib.rs when a page finishes loading.
pub fn on_page_loaded(app: &AppHandle, webview: &tauri::Webview<tauri::Wry>, url: &url::Url) {
    let url_str = url.as_str();

    // For the local shell, just emit the loaded event
    if url_str.starts_with("tauri://")
        || url_str.starts_with("http://localhost:")
        || url_str.starts_with("https://localhost:")
        || url_str == "about:blank"
    {
        let _ = webview.emit("webview:loaded", ());
        return;
    }

    // Injected on workspace (remote) pages
    let config = crate::config::load_config(app);

    if config.notifications_enabled {
        let _ = webview.eval(notification_bridge_js());
    }

    // Badge bridge wrapped in try/catch (task 11.3)
    let _ = webview.eval(&format!(
        "try {{ {} }} catch(e) {{}}",
        badge_bridge_js()
    ));

    let _ = webview.emit("webview:loaded", ());
}

/// Called when the WebView navigation fails.
pub fn on_page_load_error(app: &AppHandle, webview: &tauri::Webview<tauri::Wry>, message: String) {
    // Navigate back to local shell
    let shell_url = crate::menu::get_shell_url(app);
    if let Ok(parsed) = shell_url.parse::<url::Url>() {
        let _ = webview.navigate(parsed);
    }

    // Give shell a moment to load, then emit the error event
    let webview_clone = webview.clone();
    let msg = message.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(400));
        let _ = webview_clone.emit(
            "webview:error",
            serde_json::json!({ "message": msg }),
        );
    });
}

#[tauri::command]
pub fn navigate_workspace(app: AppHandle, url: String) -> Result<(), String> {
    load_workspace(&app, &url)
}

#[tauri::command]
pub fn clear_session(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    // Best-effort clear of JS storage and cookies
    let _ = window.eval(
        "try {
            localStorage.clear();
            sessionStorage.clear();
            document.cookie.split(';').forEach(function(c) {
                document.cookie = c.replace(/^ +/, '')
                    .replace(/=.*/, '=;expires=Thu, 01 Jan 1970 00:00:00 GMT;path=/');
            });
        } catch(e) {}",
    );

    // Delete the WebView data directory (takes effect on next WebView init)
    if let Ok(local_data) = app.path().app_local_data_dir() {
        let data_dir = local_data.join("webview-data");
        if data_dir.exists() {
            let _ = std::fs::remove_dir_all(&data_dir);
        }
    }

    // Navigate to workspace URL to show login screen
    let config = crate::config::load_config(&app);
    if !config.workspace_url.is_empty() {
        let _ = load_workspace(&app, &config.workspace_url);
    }

    Ok(())
}
