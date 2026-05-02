## 1. Config shim injection

- [x] 1.1 In `webview.rs`, add a `config_shim_js(enabled: bool) -> String` helper that returns `window.__cwDesktop = { notificationsEnabled: <bool> };`
- [x] 1.2 In `on_page_loaded`, inject the config shim before the notification bridge on every workspace page load (regardless of `notifications_enabled`)
- [x] 1.3 Remove the `if config.notifications_enabled` gate around `notification_bridge_js()` injection — always inject the bridge; the bridge itself will check the shim

## 2. Runtime toggle via Tauri command

- [x] 2.1 In `webview.rs`, add a `apply_notifications_setting(app: AppHandle, enabled: bool)` Tauri command that evals `window.__cwDesktop && (window.__cwDesktop.notificationsEnabled = <bool>)` on the main WebView
- [x] 2.2 Register `apply_notifications_setting` in the command handler list in `lib.rs`

## 3. Update notification bridge

- [x] 3.1 In `notification-bridge.js`, wrap the forwarding logic with a guard: `if (!window.__cwDesktop?.notificationsEnabled) return`
- [x] 3.2 Verify the guard uses the shim synchronously (no async IPC call)

## 4. Wire settings save to runtime toggle

- [x] 4.1 In `settings.js`, after `cmd_save_config` resolves, invoke `apply_notifications_setting` with the new `notificationsEnabled` value

## 5. Cross-platform permission handling

- [x] 5.1 In `notifications.rs`, wrap the `PermissionState` check and `request_permission` call in `#[cfg(target_os = "macos")]`
- [x] 5.2 Add a `#[cfg(not(target_os = "macos"))]` branch that returns `Ok("granted".to_string())` immediately

## 6. Verification

- [ ] 6.1 On macOS: toggle notifications off in settings, confirm a test notification from Chatwoot is not delivered without reloading the WebView
- [ ] 6.2 On macOS: toggle notifications back on, confirm notifications are delivered again immediately
- [ ] 6.3 On Windows: confirm notifications are delivered via Toast and the permission command does not error
- [ ] 6.4 On Windows: toggle notifications off and on, confirm real-time enforcement works the same as macOS
