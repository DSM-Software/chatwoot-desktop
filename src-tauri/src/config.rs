use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "config.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default)]
    pub workspace_url: String,
    #[serde(default = "default_minimize_on_close")]
    pub minimize_on_close: bool,
    #[serde(default)]
    pub open_at_login: bool,
    #[serde(default = "default_notifications_enabled")]
    pub notifications_enabled: bool,
    #[serde(default = "default_window_width")]
    pub window_width: f64,
    #[serde(default = "default_window_height")]
    pub window_height: f64,
    #[serde(default)]
    pub window_x: Option<i32>,
    #[serde(default)]
    pub window_y: Option<i32>,
}

fn default_minimize_on_close() -> bool {
    true
}

fn default_notifications_enabled() -> bool {
    true
}

fn default_window_width() -> f64 {
    1280.0
}

fn default_window_height() -> f64 {
    800.0
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            workspace_url: String::new(),
            minimize_on_close: true,
            open_at_login: false,
            notifications_enabled: true,
            window_width: 1280.0,
            window_height: 800.0,
            window_x: None,
            window_y: None,
        }
    }
}

pub fn load_config(app: &AppHandle) -> AppConfig {
    let Ok(store) = app.store(STORE_FILE) else {
        return AppConfig::default();
    };

    let workspace_url = store
        .get("workspaceUrl")
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default();

    let minimize_on_close = store
        .get("minimizeOnClose")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let open_at_login = store
        .get("openAtLogin")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let notifications_enabled = store
        .get("notificationsEnabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let window_width = store
        .get("windowWidth")
        .and_then(|v| v.as_f64())
        .unwrap_or(1280.0);

    let window_height = store
        .get("windowHeight")
        .and_then(|v| v.as_f64())
        .unwrap_or(800.0);

    let window_x = store
        .get("windowX")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);

    let window_y = store
        .get("windowY")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);

    AppConfig {
        workspace_url,
        minimize_on_close,
        open_at_login,
        notifications_enabled,
        window_width,
        window_height,
        window_x,
        window_y,
    }
}

pub fn save_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("Failed to open store: {}", e))?;

    store.set("workspaceUrl", serde_json::json!(config.workspace_url));
    store.set("minimizeOnClose", serde_json::json!(config.minimize_on_close));
    store.set("openAtLogin", serde_json::json!(config.open_at_login));
    store.set("notificationsEnabled", serde_json::json!(config.notifications_enabled));
    store.set("windowWidth", serde_json::json!(config.window_width));
    store.set("windowHeight", serde_json::json!(config.window_height));

    if let Some(x) = config.window_x {
        store.set("windowX", serde_json::json!(x));
    }
    if let Some(y) = config.window_y {
        store.set("windowY", serde_json::json!(y));
    }

    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_config(app: AppHandle) -> AppConfig {
    load_config(&app)
}

#[tauri::command]
pub fn cmd_save_config(app: AppHandle, config: AppConfig) -> Result<(), String> {
    save_config(&app, &config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_values() {
        let config = AppConfig::default();
        assert_eq!(config.workspace_url, "");
        assert!(config.minimize_on_close);
        assert!(!config.open_at_login);
        assert!(config.notifications_enabled);
        assert_eq!(config.window_width, 1280.0);
        assert_eq!(config.window_height, 800.0);
        assert!(config.window_x.is_none());
        assert!(config.window_y.is_none());
    }
}
