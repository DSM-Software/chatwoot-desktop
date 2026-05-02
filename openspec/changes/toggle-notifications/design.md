## Context

The app already persists `notificationsEnabled` in config and reads it at page load to decide whether to inject `notification-bridge.js`. The gap is that this decision is made once — at injection time — and cannot be changed mid-session. If the user toggles notifications in settings and saves, the bridge stays active (or absent) until the next WebView navigation.

A secondary gap is that `request_notification_permission` uses macOS-specific `PermissionState` logic. On Windows, `tauri-plugin-notification` does not expose an explicit permission dialog — notification access is managed via Windows system settings. Calling the macOS path on Windows either panics or silently errors.

## Goals / Non-Goals

**Goals:**
- Toggle takes effect immediately after saving settings, without reloading the WebView
- Notification permission request is safe to call on both macOS and Windows
- No new dependencies introduced

**Non-Goals:**
- Surfacing the OS-level notification permission status in the settings UI
- Windows taskbar badge count (separate concern from the notification toggle)
- Supporting browsers other than the embedded WebView

## Decisions

### Injected config shim over async IPC per notification

**Decision:** Inject `window.__cwDesktop = { notificationsEnabled: <bool> }` into the WebView on every workspace page load. The bridge reads this object instead of calling `get_config` over IPC.

**Rationale:** An async IPC call inside `TauriNotification()` would add latency to every notification and require error handling inside the bridge. The shim is synchronous, requires no extra round-trips, and can be updated at any time via `webview.eval`.

**Alternative considered:** Re-inject the full bridge on every settings save. Rejected because it re-overrides `window.Notification` redundantly when already active, and is a no-op when disabled (bridge was never injected).

### Push config update from Rust after settings save

**Decision:** Add a `apply_notifications_setting(enabled: bool)` Tauri command. When called, it runs `webview.eval("window.__cwDesktop && (window.__cwDesktop.notificationsEnabled = <bool>)")` on the main WebView.

**Rationale:** Keeps the update path in Rust, consistent with how other side effects (autostart, navigation) are applied. The frontend calls this command alongside `cmd_save_config`.

**Alternative considered:** Have `settings.js` call `webview.eval` directly via `window.__TAURI__`. Rejected because the main WebView is a separate context — `settings.js` runs in the shell WebView, not the workspace WebView.

### Platform-gated permission request

**Decision:** Wrap the `PermissionState` check and `request_permission` call in `#[cfg(target_os = "macos")]`. On all other targets, `request_notification_permission` returns `"granted"` immediately.

**Rationale:** Windows manages notification permissions at the OS level (Apps & notifications settings). There is no runtime permission dialog and `tauri-plugin-notification` does not expose one on Windows. Returning `"granted"` is accurate: if the user has denied notifications at the OS level, the plugin will silently drop them, which is the correct behavior.

## Risks / Trade-offs

- **Shim not present on error pages**: If the workspace URL fails to load, the error page is shown and the workspace WebView may not have the shim. The bridge is not injected on error pages anyway, so this is not a real risk.
- **Eval timing**: `apply_notifications_setting` runs `webview.eval` on the workspace WebView. If the WebView is mid-navigation, the eval may be dropped. This is acceptable — the next page load will inject the updated shim.
- **Windows notification silence**: If the user disables notifications at the OS level, the app has no way to detect or surface this. Notifications are silently dropped by the OS. Considered acceptable for v1.

## Migration Plan

No data migration needed. `notificationsEnabled` is already persisted. The shim injection is additive and does not change any stored config schema.
