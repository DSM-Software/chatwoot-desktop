pub mod badge;
pub mod config;
pub mod menu;
pub mod notifications;
pub mod tray;
pub mod webview;
pub mod window;

use tauri::{Manager, WindowEvent};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .invoke_handler(tauri::generate_handler![
            config::get_config,
            config::cmd_save_config,
            webview::navigate_workspace,
            webview::clear_session,
            notifications::send_notification,
            notifications::request_notification_permission,
            badge::set_badge_count,
        ])
        .setup(|app| {
            // Build and set the native application menu
            let menu = menu::build_menu(app.handle())?;
            app.set_menu(menu)?;

            // Set up the system tray icon
            tray::setup_tray(app.handle())?;

            // Create the main window (with saved geometry + navigation handler)
            window::setup_window(app.handle())?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            menu::handle_menu_event(app, event);
        })
        .on_page_load(|webview, payload| {
            let app = webview.app_handle();
            match payload.event() {
                tauri::webview::PageLoadEvent::Finished => {
                    webview::on_page_loaded(app, &webview, payload.url());
                }
                tauri::webview::PageLoadEvent::Started => {}
            }
        })
        .on_window_event(|win, event| {
            let app = win.app_handle();
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    let cfg = config::load_config(app);
                    if cfg.minimize_on_close {
                        api.prevent_close();
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.hide();
                        }
                    }
                    // Otherwise let the close proceed (app exits)
                }
                WindowEvent::Resized(_) | WindowEvent::Moved(_) => {
                    if let Some(main_win) = app.get_webview_window("main") {
                        window::persist_geometry(app, &main_win);
                    }
                }
                WindowEvent::Focused(true) => {
                    // Clear dock badge optimistically on focus (task 11.4)
                    if let Some(main_win) = app.get_webview_window("main") {
                        let _ = main_win.set_badge_label(None);
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Chatwoot desktop app");
}
