use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use url::Url;

use crate::config::{load_config, save_config};

pub fn setup_window(app: &AppHandle) -> tauri::Result<()> {
    let config = load_config(app);

    // Initialise workspace URL state used by the navigation handler
    crate::webview::update_workspace_url(&config.workspace_url);
    let workspace_url_ref = crate::webview::workspace_url_state().clone();
    let app_for_nav = app.clone();

    let mut builder =
        WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
            .title("Chatwoot")
            .inner_size(config.window_width, config.window_height)
            .min_inner_size(800.0, 600.0)
            .resizable(true)
            .data_directory(
                app.path()
                    .app_local_data_dir()
                    .unwrap_or_default()
                    .join("webview-data"),
            )
            // Navigation interception: open external links in system browser
            .on_navigation(move |url: &Url| {
                let url_str = url.as_str();

                // Always allow local shell URLs
                if url_str.starts_with("tauri://")
                    || url_str.starts_with("http://localhost:")
                    || url_str.starts_with("https://localhost:")
                    || url_str == "about:blank"
                {
                    return true;
                }

                let stored = workspace_url_ref.lock().unwrap().clone();

                // Allow all while no workspace is configured
                if stored.is_empty() {
                    return true;
                }

                let workspace_host = Url::parse(&stored)
                    .ok()
                    .and_then(|u| u.host_str().map(String::from));

                let nav_host = url.host_str().map(String::from);

                if workspace_host == nav_host {
                    return true; // Same domain — allow in WebView
                }

                // External URL — open in system browser, block WebView
                let ext_url = url.to_string();
                let app_clone = app_for_nav.clone();
                tauri::async_runtime::spawn(async move {
                    use tauri_plugin_opener::OpenerExt;
                    let _ = app_clone.opener().open_url(&ext_url, None::<&str>);
                });

                false
            });

    if let (Some(x), Some(y)) = (config.window_x, config.window_y) {
        builder = builder.position(x as f64, y as f64);
    } else {
        builder = builder.center();
    }

    builder.build()?;

    Ok(())
}

pub fn persist_geometry(app: &AppHandle, window: &tauri::WebviewWindow) {
    let Ok(size) = window.inner_size() else {
        return;
    };
    let Ok(pos) = window.outer_position() else {
        return;
    };

    let mut config = load_config(app);
    config.window_width = size.width as f64;
    config.window_height = size.height as f64;
    config.window_x = Some(pos.x);
    config.window_y = Some(pos.y);

    let _ = save_config(app, &config);
}

pub fn get_main_window(app: &AppHandle) -> Option<tauri::WebviewWindow> {
    app.get_webview_window("main")
}

pub fn show_and_focus(app: &AppHandle) {
    if let Some(window) = get_main_window(app) {
        let _ = window.show();
        let _ = window.set_focus();
        #[cfg(target_os = "macos")]
        let _ = app.show();
    }
}
