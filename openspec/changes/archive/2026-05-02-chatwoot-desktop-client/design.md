## Context

This project is a net-new macOS desktop client for Chatwoot, built with Tauri v2. The app wraps the Chatwoot web frontend in a native WebView, providing session persistence, native notifications, configurable workspace URL, and a professional desktop experience. There is no existing codebase — this design establishes the foundational architecture.

Target: macOS first, with Windows/Linux compatibility achievable without major rework due to Tauri's cross-platform nature.

## Goals / Non-Goals

**Goals:**
- Deliver a working, installable macOS app that wraps Chatwoot in a native window
- Persist WebView session (cookies + localStorage) so users never need to re-login
- Allow workspace URL to be configured and changed at any time
- Provide native macOS integration: menu bar, system tray, notifications, badge, open-at-login
- External link interception: non-workspace links open in the system browser
- Graceful loading and error screens with retry/settings actions
- Minimal, secure Tauri CSP and permission surface

**Non-Goals:**
- Building a native Chatwoot UI from scratch (we render Chatwoot's web app)
- Windows or Linux builds in v1 (structure supports it, but not tested)
- Custom Chatwoot API integration or websocket handling at the Rust layer
- User account management or multi-account switching

## Decisions

### 1. Tauri v2 with vanilla JavaScript frontend (no framework)

**Decision**: Use Tauri v2 with a plain HTML/CSS/JS frontend (no React/Vue).

**Rationale**: The frontend consists of only 3 screens — setup, settings, and error. A full framework adds build complexity without benefit. Vanilla JS keeps the codebase simple and the bundle tiny. Tauri v2 provides the best macOS integration via its plugin ecosystem.

**Alternatives considered**:
- Vue 3 with Vite: familiar DX but overkill for 3 static screens
- React: same concern, heavier dependency footprint

---

### 2. WebView for Chatwoot rendering

**Decision**: Load the configured Chatwoot URL in Tauri's native WebView (WKWebView on macOS).

**Rationale**: Chatwoot is a full SPA. Embedding the WebView gives us 100% feature parity with the browser, free session persistence via WKWebView's cookie store, and zero maintenance overhead for Chatwoot feature updates.

**Trade-off**: We cannot control Chatwoot internals. Any Chatwoot changes that break the WebView experience require workarounds at our layer only (JS injection or error handling).

---

### 3. `tauri-plugin-store` for configuration persistence

**Decision**: Use `tauri-plugin-store` (JSON file in app data directory) for storing workspace URL, window geometry, and user preferences.

**Rationale**: Simple, Tauri-native, no external DB. The store is a plain JSON file the user can inspect or reset manually. Alternatives (SQLite, OS keychain) are over-engineered for this use case.

---

### 4. Session persistence via WebView data directory

**Decision**: Rely on WKWebView's native cookie/localStorage persistence by setting `data_directory` in `tauri.conf.json`.

**Rationale**: WKWebView already persists session data to disk by default when a data directory is configured. No custom code needed. On restart, the WebView restores the session automatically.

---

### 5. External link interception via navigation event handler

**Decision**: Intercept `navigation-started` events in the WebView and use `tauri-plugin-shell` to open external links in the default browser.

**Rationale**: Chatwoot opens external links (attachments, references) via standard `<a target="_blank">` or `window.open`. Intercepting at the Tauri navigation layer is the safest, framework-agnostic approach.

**Rule**: A link is "external" if its hostname differs from the configured workspace URL's hostname.

---

### 6. Notifications via `tauri-plugin-notification` + JS bridge

**Decision**: Inject a small JS snippet into the WebView that intercepts `Notification` constructor calls from Chatwoot's web code and proxies them to Tauri's native notification plugin.

**Rationale**: Chatwoot already uses the Web Notifications API to show browser notifications. WKWebView may not forward these to macOS unless the app has the right entitlements. Proxying through Tauri ensures they appear as native macOS notifications and can trigger focus behavior.

**Alternative**: Rely purely on WebView's notification forwarding. This is unreliable on macOS without special entitlements.

---

### 7. System tray with hide-to-tray behavior

**Decision**: Implement a system tray icon using `tauri-plugin-tray`. Closing the window hides it (sends to tray) rather than quitting. Tray menu provides Show/Quit.

**Rationale**: Support agents expect the app to keep running in the background so notifications arrive even when the window is closed. This matches the behavior of apps like Slack and Discord.

**Optional for v1**: If tray increases complexity significantly, it can ship as a disabled-by-default feature behind a settings toggle.

---

### 8. Security: allowlist-based navigation

**Decision**: Configure Tauri's CSP and navigation allowlist to only permit navigation within the configured Chatwoot domain. All other navigations are intercepted and either opened externally or blocked.

**Rationale**: Prevents the WebView from being redirected to arbitrary URLs, which could expose session cookies to untrusted sites.

---

### 9. App architecture: modular Rust + single JS bundle

**Decision**: Organize Rust code into focused modules (`config.rs`, `window.rs`, `menu.rs`, `tray.rs`, `notifications.rs`). Frontend is a single `index.html` with embedded CSS/JS for each screen (setup, settings, error) toggled via JS show/hide.

**Rationale**: Keeps the project small and readable. All screens share one HTML file to avoid Tauri multi-window complexity for simple UI states.

## Risks / Trade-offs

- **Chatwoot JS changes** → The JS injection for notification proxying may break if Chatwoot changes how it calls the Notifications API. Mitigation: keep the injection minimal and version-pin Chatwoot if self-hosted.
- **WKWebView session loss** → If the user manually deletes app data or the store gets corrupted, session is lost. Mitigation: the "Clear session" settings button handles this gracefully with a user-facing confirmation.
- **Badge count accuracy** → Reading unread counts from the WebView DOM requires polling or Chatwoot-specific selectors that may change. Mitigation: implement badge update as best-effort; if selectors break, badge simply shows nothing (no crash).
- **macOS notarization** → Distributing outside the App Store requires Apple notarization. Tauri v2 supports this workflow, but it requires an Apple Developer account. Mitigation: document the notarization step; dev builds can run after gatekeeper bypass.
- **Open-at-login reliability** → `tauri-plugin-autostart` uses LaunchAgents on macOS, which requires no special entitlements but can be affected by macOS updates. Mitigation: expose the setting in the UI with a note that the user can verify in System Settings > Login Items.

## Open Questions

- Should the app support multiple workspace accounts in a future version? (Current design assumes one URL; a multi-account feature would require per-account WebView isolation.)
- Is there a preferred Chatwoot-specific API endpoint we can poll for unread counts, or must we rely on DOM scraping for the badge?
