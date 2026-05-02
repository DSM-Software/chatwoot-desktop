# Chatwoot Desktop

A native macOS desktop client for [Chatwoot](https://www.chatwoot.com), built with [Tauri v2](https://tauri.app) and vanilla JavaScript.

The app wraps the Chatwoot web app in a persistent, native macOS window with session persistence, desktop notifications, dock badge support, and a system tray icon.

---

## Prerequisites

| Tool | Version |
|------|---------|
| Rust | ≥ 1.77 (`rustup update stable`) |
| Node.js | ≥ 18 |
| npm | ≥ 9 |
| Xcode Command Line Tools | latest (`xcode-select --install`) |

Install the Tauri CLI:

```bash
npm install           # installs @tauri-apps/cli and vite
```

---

## Development

```bash
npm run tauri dev
```

This starts the Vite dev server on `http://localhost:5173` and launches the app with hot-reload for the frontend. Rust changes require a restart.

---

## Build (macOS)

```bash
npm run tauri build
```

Produces:
- `src-tauri/target/release/bundle/macos/Chatwoot.app`
- `src-tauri/target/release/bundle/dmg/Chatwoot_*.dmg`

For a universal binary (arm64 + x86_64):

```bash
npm run tauri build -- --target universal-apple-darwin
```

> Requires both `aarch64-apple-darwin` and `x86_64-apple-darwin` Rust targets:
> ```bash
> rustup target add aarch64-apple-darwin x86_64-apple-darwin
> ```

---

## Replacing the App Icon

1. Replace the source icon file (e.g. a 1024×1024 PNG named `app-icon.png`)
2. Run:
   ```bash
   npm run tauri icon app-icon.png
   ```
   This generates all required sizes in `src-tauri/icons/`.

---

## First Run

On first launch, the app shows a setup screen. Enter your Chatwoot workspace URL (e.g. `https://app.chatwoot.com` or your self-hosted instance). The URL is saved locally and restored on subsequent launches.

---

## Settings

Open Settings via **View → Settings** or **⌘,**.

| Setting | Description |
|---------|-------------|
| Workspace URL | Your Chatwoot instance URL |
| Open at login | Register as a macOS Login Item |
| Keep running when closed | Hide to tray instead of quitting |
| Desktop notifications | Forward Chatwoot notifications to macOS |
| Clear session | Sign out and delete stored session data |

---

## Architecture

```
chatwoot-desktop/
├── src/                       # Vanilla JS/HTML/CSS frontend (shell UI)
│   ├── index.html             # Single HTML shell with all views
│   ├── css/main.css           # System-adaptive styles
│   └── js/
│       ├── api.js             # Tauri IPC wrapper
│       ├── router.js          # View show/hide
│       ├── validate-url.js    # Shared URL validation
│       ├── setup.js           # First-run setup screen
│       ├── app.js             # App/loading view controller
│       ├── error.js           # Error screen handler
│       ├── settings.js        # Settings screen
│       ├── notification-bridge.js  # Injected: overrides window.Notification
│       └── badge-bridge.js         # Injected: polls DOM for unread count
└── src-tauri/                 # Rust / Tauri backend
    ├── src/
    │   ├── main.rs            # Entry point
    │   ├── lib.rs             # App setup, plugin registration
    │   ├── config.rs          # AppConfig struct, load/save via tauri-plugin-store
    │   ├── window.rs          # Window creation, geometry persistence
    │   ├── menu.rs            # Native macOS menu
    │   ├── tray.rs            # System tray icon
    │   ├── webview.rs         # Navigation, external link interception, bridge injection
    │   ├── notifications.rs   # Native notification commands
    │   └── badge.rs           # Dock badge command
    └── tauri.conf.json        # App configuration
```

---

## Known Limitations

### WebView notification forwarding
Chatwoot uses the browser `Notification` API. On macOS, WKWebView does not reliably forward these to the system notification center without special entitlements. This app overrides `window.Notification` in the WebView via JS injection and forwards calls to `tauri-plugin-notification`. This approach requires macOS notification permission to be granted.

**Limitation**: The JS injection fires after `pageLoadFinished`, which means very early notifications (before the Chatwoot app is fully bootstrapped) may use the original `Notification` constructor. This is rare in practice.

### Badge DOM selector fragility
The dock badge is updated by polling `document.querySelector('[data-key="conversations-badge"]')` every 5 seconds in the injected `badge-bridge.js`. If Chatwoot changes this DOM structure, the badge will silently stop updating. The app will not crash; the badge simply retains its last value or shows nothing.

To fix a broken selector, update `BADGE_SELECTORS` in `src/js/badge-bridge.js`.

### Notarization for distribution
Distributing outside the Mac App Store requires notarization by Apple. Tauri v2 supports this workflow. You will need:
- An Apple Developer account
- A Developer ID Application certificate
- Xcode tools with `notarytool`

See [Tauri's notarization guide](https://tauri.app/distribute/sign/macos/) for full instructions.

### Session clearing
The "Clear session" action clears `localStorage`, `sessionStorage`, and cookies accessible via JavaScript. It also deletes the app's WebView data directory on disk. However, WKWebView may cache some data in memory for the current session. A full session clear takes full effect after restarting the app.

### IPC security
`dangerousRemoteUrlIpcAccess` is currently set to allow any HTTPS/HTTP domain. For production deployments, update `tauri.conf.json` to restrict IPC access to only your workspace domain:

```json
"dangerousRemoteUrlIpcAccess": [
  {
    "scheme": "https",
    "domain": "your-workspace.chatwoot.com",
    "windows": ["main"],
    "plugins": [],
    "enableBrownfield": false
  }
]
```
