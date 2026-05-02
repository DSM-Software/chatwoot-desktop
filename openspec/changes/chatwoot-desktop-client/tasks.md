## 1. Project Scaffolding

- [x] 1.1 Initialize Tauri v2 project with `cargo tauri init` (app name: Chatwoot, bundle id: com.chatwoot.desktop)
- [x] 1.2 Set up Vite as the frontend dev server and configure `tauri.conf.json` `devUrl` and `frontendDist`
- [x] 1.3 Add Tauri plugins to `Cargo.toml`: `tauri-plugin-store`, `tauri-plugin-notification`, `tauri-plugin-autostart`, `tauri-plugin-shell`
- [x] 1.4 Register all plugins in `src-tauri/src/main.rs`
- [x] 1.5 Create project directory structure: `src/` (frontend), `src-tauri/src/` modules, `icons/`
- [x] 1.6 Add placeholder app icons and run `tauri icon` to generate all required sizes
- [x] 1.7 Verify `cargo tauri dev` launches a blank window on macOS

## 2. Configuration Store

- [x] 2.1 Create `src-tauri/src/config.rs` — define `AppConfig` struct with fields: `workspace_url`, `minimize_on_close`, `open_at_login`, `notifications_enabled`, `window_width`, `window_height`, `window_x`, `window_y`
- [x] 2.2 Implement `load_config()` and `save_config()` functions using `tauri-plugin-store` (store file: `config.json`)
- [x] 2.3 Expose Tauri commands `get_config` and `save_config` callable from the frontend
- [x] 2.4 Write unit test: default config is returned when store file does not exist

## 3. App Shell — Window & Menu

- [x] 3.1 Create `src-tauri/src/window.rs` — function to build the main window with `tauri::WebviewWindowBuilder`, applying stored geometry or defaults (1280×800, min 800×600)
- [x] 3.2 Implement window geometry persistence: listen to `on_window_event` for `CloseRequested` and `Moved`/`Resized` events; write geometry to config store before close
- [x] 3.3 Create `src-tauri/src/menu.rs` — build native macOS menu with App / View (Reload ⌘R, Back ⌘[, Forward ⌘], Settings ⌘,) / Window sections
- [x] 3.4 Wire menu events: Reload → emit `menu:reload`, Settings → emit `menu:settings`, Quit → `app.exit(0)`
- [x] 3.5 Handle `CloseRequested` window event: if `minimize_on_close` is true, hide window instead of closing; if false, exit

## 4. System Tray

- [x] 4.1 Create `src-tauri/src/tray.rs` — build tray icon with `tauri-plugin-tray` using the app icon
- [x] 4.2 Add tray menu items: "Open Chatwoot" and "Quit"
- [x] 4.3 Wire "Open Chatwoot" to show and focus the main window
- [x] 4.4 Wire "Quit" to `app.exit(0)`
- [x] 4.5 Initialize tray in `main.rs` after app setup

## 5. Frontend — Shared Layout & Routing

- [x] 5.1 Create `src/index.html` as the single HTML shell with three named views: `#view-setup`, `#view-app`, `#view-error`
- [x] 5.2 Create `src/js/router.js` — show/hide views by ID; expose `navigate(viewId)` function
- [x] 5.3 Create `src/css/main.css` — minimal, clean styles: dark/light system-adaptive background, Inter or system font, no browser UI chrome
- [x] 5.4 Create `src/js/api.js` — thin wrapper around `window.__TAURI__.core.invoke` for calling Tauri commands

## 6. First-Run Setup Screen

- [x] 6.1 Build `#view-setup` HTML: logo, heading "Connect your Chatwoot", URL input field, submit button
- [x] 6.2 Create `src/js/setup.js` — on mount, call `get_config`; if `workspace_url` is set, call `router.navigate('#view-app')`
- [x] 6.3 Implement URL validation in `setup.js`: reject empty/malformed input, strip trailing slash, warn on HTTP
- [x] 6.4 On valid submit: call `save_config` with new URL, navigate to `#view-app`

## 7. WebView Manager

- [x] 7.1 Create `src-tauri/src/webview.rs` — function `load_workspace(window, url)` that navigates the WebView to the given URL
- [x] 7.2 Configure WebView data directory in `tauri.conf.json` to persist cookies and localStorage between sessions
- [x] 7.3 Implement navigation event handler in `webview.rs`: intercept `NavigationStarted` events; compare hostname to stored workspace URL; call `shell.open()` for external URLs
- [x] 7.4 Expose Tauri command `navigate_workspace` that frontend can call to change the loaded URL
- [x] 7.5 Expose Tauri command `clear_session` that deletes WebView data directory contents and reloads

## 8. Loading & Error States

- [x] 8.1 Build `#view-error` HTML: icon, "Could not connect" heading, error detail text, "Try Again" and "Change URL" buttons
- [x] 8.2 Create `src/js/error.js` — handle `Try Again` (navigate to `#view-app`, trigger reload) and `Change URL` (navigate to `#view-settings`)
- [x] 8.3 In `src/js/app.js` (controls `#view-app`): show loading overlay while WebView page is loading; listen for Tauri `webview:loaded` and `webview:error` events to dismiss overlay or show `#view-error`
- [x] 8.4 Emit `webview:loaded` and `webview:error` events from Rust `webview.rs` by listening to `WebviewEvent::PageLoadFinished` and `WebviewEvent::NavigationFailed`

## 9. Settings Screen

- [x] 9.1 Build `#view-settings` HTML: workspace URL field, toggles for open-at-login / minimize-on-close / notifications, "Clear session" button, Save and Cancel buttons
- [x] 9.2 Create `src/js/settings.js` — on mount, call `get_config` and populate all fields from current config
- [x] 9.3 Implement URL validation (same logic as setup.js — extract to `src/js/validate-url.js` shared module)
- [x] 9.4 On Save: call `save_config`, apply autostart change via `autostart` plugin command, navigate back and reload WebView if URL changed
- [x] 9.5 On "Clear session": show native confirmation dialog via `dialog.ask()`; on confirm, call `clear_session` command
- [x] 9.6 Wire ⌘, menu event and `menu:settings` Tauri event to `router.navigate('#view-settings')`

## 10. Notifications

- [x] 10.1 Create `src-tauri/src/notifications.rs` — expose Tauri command `send_notification(title, body)` using `tauri-plugin-notification`
- [x] 10.2 Implement `request_notification_permission()` command that requests macOS notification permission if not already granted
- [x] 10.3 Create `src/js/notification-bridge.js` — override `window.Notification` constructor; if `notifications_enabled` config is true, call `send_notification` Tauri command instead of creating a browser notification
- [x] 10.4 Inject `notification-bridge.js` into the WebView on page load via `webview.eval()` after `webview:loaded` event
- [x] 10.5 Listen for `notification-action-performed` Tauri event in `app.js`; on click, show and focus the main window

## 11. Dock Badge (Best-Effort)

- [x] 11.1 Create `src-tauri/src/badge.rs` — expose Tauri command `set_badge_count(count: u32)` that calls `app.set_badge_count(Some(count))` on macOS
- [x] 11.2 Create `src/js/badge-bridge.js` — poll DOM every 5 seconds for the Chatwoot unread count element (selector: `[data-key="conversations-badge"]` or equivalent); call `set_badge_count` with the parsed integer value
- [x] 11.3 Inject `badge-bridge.js` after `webview:loaded`; wrap in try/catch so failures are silent
- [x] 11.4 Call `set_badge_count(0)` when the window gains focus (optimistic clear)

## 12. Security Hardening

- [x] 12.1 Set Tauri CSP in `tauri.conf.json` to allow only the workspace domain for `connect-src` and `frame-src`
- [x] 12.2 Configure `dangerousRemoteUrlIpcAccess` only for the workspace origin; deny IPC from all other origins
- [x] 12.3 Audit plugin permissions in `capabilities/*.json` — remove any capability not required by the app
- [x] 12.4 Add navigation allowlist: reject navigation to any URL not matching workspace hostname pattern
- [x] 12.5 Add `Content-Security-Policy` meta tag to `index.html` for the frontend shell

## 13. Build & Distribution

- [x] 13.1 Add `tauri.conf.json` bundle configuration: `targets: ["dmg", "app"]`, category `"Productivity"`, copyright, version
- [x] 13.2 Verify `cargo tauri build` produces a `.app` and `.dmg` for macOS (arm64 + x86_64)
- [x] 13.3 Write `README.md` with: prerequisites (Rust, Node, Xcode tools), `npm run dev` dev instructions, `npm run build` macOS build instructions, icon replacement steps, and known limitations
- [x] 13.4 Document technical limitations in `README.md`: WebView notification forwarding behavior, badge DOM selector fragility, notarization requirements for distribution
