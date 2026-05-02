# Chatwoot Desktop

A native desktop client for [Chatwoot](https://www.chatwoot.com), built with [Tauri v2](https://tauri.app) and vanilla JavaScript.

The app wraps the Chatwoot web app in a persistent native window with session persistence, desktop notifications, dock/taskbar badge support, and a system tray icon.

**Supported platforms:** macOS (Intel + Apple Silicon) · Windows 11

---

## Features

- Persistent session — stay signed in between launches
- Native desktop notifications forwarded from the Chatwoot web app
- Dock badge (macOS) / taskbar badge (Windows) showing unread conversation count
- System tray icon with quick actions
- Open at login option
- Hide to tray instead of quitting
- First-run setup screen for any Chatwoot instance (cloud or self-hosted)

---

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | ≥ 1.77 | [rustup.rs](https://rustup.rs) |
| Node.js | ≥ 18 | [nodejs.org](https://nodejs.org) |
| npm | ≥ 9 | bundled with Node.js |
| Xcode CLT *(macOS only)* | latest | `xcode-select --install` |
| WebView2 *(Windows only)* | latest | pre-installed on Windows 11 |

---

## Getting Started

```bash
git clone https://github.com/YOUR_USERNAME/chatwoot-desktop.git
cd chatwoot-desktop
npm install
npm run tauri dev
```

This starts the Vite dev server and launches the app with hot-reload for the frontend. Rust changes require a restart.

---

## Building

### macOS

```bash
npm run tauri build
```

Produces:
- `src-tauri/target/release/bundle/macos/Chatwoot.app`
- `src-tauri/target/release/bundle/dmg/Chatwoot_*.dmg`

Universal binary (Intel + Apple Silicon):

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run tauri build -- --target universal-apple-darwin
```

### Windows

```bash
npm run tauri build
```

Produces:
- `src-tauri/target/release/bundle/nsis/Chatwoot_*_x64-setup.exe`

### Automated releases (GitHub Actions)

Pushing a version tag triggers the CI workflow, which builds for both platforms and creates a GitHub Release with the installers attached:

```bash
git tag v1.0.0
git push origin v1.0.0
```

---

## Architecture

```
chatwoot-desktop/
├── src/                            # Frontend (vanilla JS/HTML/CSS shell)
│   ├── index.html                  # Single HTML shell with all views
│   ├── css/main.css                # System-adaptive styles
│   └── js/
│       ├── api.js                  # Tauri IPC wrapper
│       ├── router.js               # View show/hide logic
│       ├── validate-url.js         # Shared URL validation
│       ├── setup.js                # First-run setup screen
│       ├── app.js                  # App/loading view controller
│       ├── error.js                # Error screen handler
│       ├── settings.js             # Settings screen
│       ├── notification-bridge.js  # Injected: overrides window.Notification
│       └── badge-bridge.js         # Injected: polls DOM for unread count
└── src-tauri/                      # Rust / Tauri backend
    ├── src/
    │   ├── main.rs                 # Entry point
    │   ├── lib.rs                  # App setup, plugin registration
    │   ├── config.rs               # AppConfig struct (tauri-plugin-store)
    │   ├── window.rs               # Window creation, geometry persistence
    │   ├── menu.rs                 # Native menu bar
    │   ├── tray.rs                 # System tray icon
    │   ├── webview.rs              # Navigation, link interception, JS injection
    │   ├── notifications.rs        # Native notification commands
    │   └── badge.rs                # Dock/taskbar badge command
    └── tauri.conf.json             # App configuration
```

---

## Known Limitations

**WebView notification timing** — The JS notification bridge is injected after `pageLoadFinished`. Notifications fired before Chatwoot fully bootstraps may use the original browser `Notification` constructor and not reach the system notification center.

**Badge selector fragility** — The dock/taskbar badge is updated by polling `document.querySelector('[data-key="conversations-badge"]')` every 5 seconds. If Chatwoot changes this DOM structure, the badge silently stops updating. To fix it, update `BADGE_SELECTORS` in `src/js/badge-bridge.js`.

**macOS notarization** — Distributing outside the Mac App Store requires Apple notarization. See [Tauri's notarization guide](https://tauri.app/distribute/sign/macos/).

**IPC security** — `dangerousRemoteUrlIpcAccess` currently allows any HTTPS/HTTP domain. For production deployments, restrict it in `tauri.conf.json` to your specific workspace domain.

---

## Contributing

Contributions are welcome. Please read the guidelines below before opening a PR.

### Reporting bugs

Open an issue and include:
- OS and version (e.g. macOS 15.1, Windows 11 23H2)
- App version
- Steps to reproduce
- What you expected vs. what happened
- Relevant logs (open DevTools via **View → Developer Tools**)

### Suggesting features

Open an issue with the `enhancement` label. Describe the use case and why the feature belongs in a desktop wrapper rather than in the Chatwoot web app itself.

### Submitting a pull request

1. Fork the repository and create a branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. Make your changes. Keep commits focused — one logical change per commit.

3. Test on at least the platform you changed. If you touched Rust code, test on both macOS and Windows if possible.

4. Open a pull request against `main` with a clear description of what changed and why.

### Development tips

- Frontend changes (JS/CSS/HTML) hot-reload automatically — no restart needed.
- Rust changes require restarting `npm run tauri dev`.
- Use **View → Developer Tools** in the running app to inspect the frontend shell.
- The Chatwoot WebView is a separate context — use the DevTools console and inject `window.__TAURI__` calls to debug IPC.

### Replacing the app icon

```bash
npm run tauri icon path/to/icon-1024x1024.png
```

This regenerates all required sizes in `src-tauri/icons/`.

### Code style

- Rust: standard `rustfmt` formatting (`cargo fmt`)
- JavaScript: no framework, no build-time transpilation — keep it vanilla
- Avoid adding dependencies unless strictly necessary

---

## License

MIT
