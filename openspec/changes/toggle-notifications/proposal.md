## Why

The notification toggle exists in settings and is persisted to config, but it only takes effect on the next page load — toggling it during an active session has no effect until the WebView reloads. Additionally, the notification permission request uses macOS-specific logic that fails silently on Windows, and the bridge injection path does not account for platform differences.

## What Changes

- Inject a shared config shim (`window.__cwDesktop`) into the WebView on every workspace page load, exposing `notificationsEnabled` as a live-readable value
- When settings are saved, push the updated `notificationsEnabled` value into the WebView context immediately via `webview.eval` — no reload required
- Update `notification-bridge.js` to read `window.__cwDesktop.notificationsEnabled` before forwarding each notification, so the toggle takes effect in real time
- Gate macOS-specific notification permission request behind `#[cfg(target_os = "macos")]` so Windows does not attempt an unsupported permission dialog (Windows manages notification access via system settings)

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `notifications`: Update requirements to cover cross-platform behavior (macOS + Windows), runtime toggle enforcement without reload, and platform-specific permission handling
- `settings-ui`: Update the notifications toggle save scenario to reflect that the change takes effect immediately in the active session

## Impact

- `src-tauri/src/webview.rs` — inject config shim on page load; emit updated `notificationsEnabled` after settings save
- `src-tauri/src/notifications.rs` — gate permission request on `#[cfg(target_os = "macos")]`; return `"granted"` directly on Windows
- `src/js/notification-bridge.js` — read `window.__cwDesktop.notificationsEnabled` before forwarding each notification
- `src/js/settings.js` — after saving, invoke a new `apply_notifications_setting` command (or eval directly) to update the WebView config shim live
- `src-tauri/tauri.conf.json` — no changes required; `tauri-plugin-notification` already targets both platforms
