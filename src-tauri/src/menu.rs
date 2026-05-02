use tauri::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu},
    AppHandle, Emitter, Manager, Wry,
};

pub fn build_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    // App menu
    let about = PredefinedMenuItem::about(app, Some("About Chatwoot"), None)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit Chatwoot", true, Some("cmd+q"))?;
    let app_menu = Submenu::with_id_and_items(
        app,
        "app",
        "Chatwoot",
        true,
        &[&about, &separator1, &quit],
    )?;

    // View menu
    let reload = MenuItem::with_id(app, "reload", "Reload", true, Some("cmd+r"))?;
    let back = MenuItem::with_id(app, "back", "Back", true, Some("cmd+["))?;
    let forward = MenuItem::with_id(app, "forward", "Forward", true, Some("cmd+]"))?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, Some("cmd+,"))?;
    let view_menu = Submenu::with_id_and_items(
        app,
        "view",
        "View",
        true,
        &[&reload, &back, &forward, &separator2, &settings],
    )?;

    // Window menu
    let minimize = PredefinedMenuItem::minimize(app, Some("Minimize"))?;
    let zoom = PredefinedMenuItem::maximize(app, Some("Zoom"))?;
    let close = PredefinedMenuItem::close_window(app, Some("Close"))?;
    let window_menu = Submenu::with_id_and_items(
        app,
        "window",
        "Window",
        true,
        &[&minimize, &zoom, &close],
    )?;

    Menu::with_items(app, &[&app_menu, &view_menu, &window_menu])
}

pub fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "quit" => {
            app.exit(0);
        }
        "reload" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("menu:reload", ());
            }
        }
        "settings" => {
            navigate_to_settings(app);
        }
        "back" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("window.history.back()");
            }
        }
        "forward" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval("window.history.forward()");
            }
        }
        _ => {}
    }
}

pub fn navigate_to_settings(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        // Emit event so if shell is loaded, it can navigate to settings view
        let _ = window.emit("menu:settings", ());
        // Also navigate back to shell with settings hash if we're on a remote URL
        let shell_url = get_shell_url(app);
        let settings_url = format!("{}#view-settings", shell_url);
        let _ = window.eval(&format!(
            "if (window.__isChatwootShell) {{ window.__router && window.__router.navigate('#view-settings'); }} else {{ window.location.href = '{}'; }}",
            settings_url
        ));
    }
}

pub fn get_shell_url(_app: &AppHandle) -> String {
    if cfg!(debug_assertions) {
        "http://localhost:5173".to_string()
    } else {
        "tauri://localhost".to_string()
    }
}
