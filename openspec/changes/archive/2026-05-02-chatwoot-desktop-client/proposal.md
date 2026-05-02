## Why

Support teams using Chatwoot currently depend on browser tabs, which lack persistent sessions, native notifications, and the focused experience of a dedicated app. A native macOS desktop client built with Tauri wraps the Chatwoot web app in a reliable, professional shell — giving agents a stable, distraction-free tool they can use daily without browser overhead.

## What Changes

- **New project**: A Tauri-based desktop application is introduced at the root of this repository
- Bootstraps a native macOS window that loads a configurable Chatwoot URL via WebView
- First-run setup screen to configure the workspace URL (self-hosted or cloud)
- Persistent storage for URL config and window geometry (size/position)
- Session persistence via WebView cookies and localStorage (no login required on restart)
- Native application menu with Reload, Back, Forward, Settings, and Quit
- External link interception — non-Chatwoot links open in the system browser
- Loading screen and friendly error screen with retry/settings actions
- Settings screen: workspace URL, open-at-login, minimize-on-close, notifications toggle, clear session
- System tray icon with open/quit actions
- macOS badge support for unread conversation counts (best-effort via WebView JS bridge)
- Desktop notifications with click-to-focus behavior
- Security: minimal Tauri permissions, allowlisted domains, no arbitrary navigation
- Build artifacts and instructions for macOS (`.app`, `.dmg`) and dev environment

## Capabilities

### New Capabilities

- `app-shell`: Main Tauri application bootstrap, window lifecycle, and native macOS integration (menu bar, tray, badge, open-at-login)
- `workspace-config`: First-run and settings screens for configuring and validating the Chatwoot workspace URL, with local persistence
- `webview-manager`: WebView setup, session persistence, external link interception, and loading/error state management
- `notifications`: Desktop notification delivery and click-to-focus behavior, bridging Chatwoot web events to native macOS notifications
- `settings-ui`: Settings screen exposing user-configurable preferences (URL, startup behavior, minimize-on-close, notifications, clear session)

### Modified Capabilities

## Impact

- Introduces a full Tauri Rust + frontend project (new `src-tauri/` and `src/` directories)
- Adds `Cargo.toml`, `tauri.conf.json`, and frontend tooling (Vite + vanilla JS or minimal framework)
- No existing application code is modified — this is a net-new desktop client project
- macOS entitlements required for notifications and keychain-adjacent storage
- Dependencies: Tauri v2, tauri-plugin-store, tauri-plugin-notification, tauri-plugin-autostart, tauri-plugin-shell
