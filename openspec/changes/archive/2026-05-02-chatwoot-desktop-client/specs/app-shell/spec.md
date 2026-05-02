## ADDED Requirements

### Requirement: Application bootstrap and main window
The app SHALL initialize a single main window on launch with a default size of 1280×800 pixels, a minimum size of 800×600, full resize support, and a title of "Chatwoot".

#### Scenario: First launch creates window at default size
- **WHEN** the app is launched for the first time
- **THEN** a window opens at 1280×800 centered on the primary display

#### Scenario: Subsequent launches restore last window geometry
- **WHEN** the app is launched after a previous session
- **THEN** the window opens at the same position and size as when it was last closed

#### Scenario: Window size is persisted on close
- **WHEN** the user resizes or moves the window and then closes it
- **THEN** the new geometry is written to the local store before the window is destroyed

---

### Requirement: Native macOS application menu
The app SHALL provide a native macOS menu bar with the following structure:
- **App menu**: About, Separator, Quit
- **View menu**: Reload (⌘R), Back (⌘[), Forward (⌘]), Separator, Settings (⌘,)
- **Window menu**: Minimize (⌘M), Zoom, Close (⌘W)

#### Scenario: Reload triggers WebView reload
- **WHEN** the user selects View → Reload or presses ⌘R
- **THEN** the WebView reloads the current Chatwoot URL

#### Scenario: Settings opens settings screen
- **WHEN** the user selects View → Settings or presses ⌘,
- **THEN** the settings screen is displayed

#### Scenario: Quit terminates the process
- **WHEN** the user selects App → Quit or presses ⌘Q
- **THEN** the application process exits cleanly

---

### Requirement: System tray icon
The app SHALL install a system tray icon that persists while the process is running, even when the main window is hidden.

#### Scenario: Tray icon is visible when window is closed
- **WHEN** the user closes the main window
- **THEN** the process continues running and the tray icon remains visible

#### Scenario: Tray menu shows app name and actions
- **WHEN** the user clicks the tray icon
- **THEN** a menu appears with "Open Chatwoot" and "Quit" options

#### Scenario: "Open Chatwoot" brings window to front
- **WHEN** the user selects "Open Chatwoot" from the tray menu
- **THEN** the main window becomes visible and gains focus

#### Scenario: "Quit" from tray exits the process
- **WHEN** the user selects "Quit" from the tray menu
- **THEN** the application process exits cleanly

---

### Requirement: Minimize-on-close behavior
When the "minimize on close" setting is enabled, closing the window SHALL hide it to the tray rather than quitting.

#### Scenario: Window hides to tray when setting is enabled
- **WHEN** the user closes the window and "minimize on close" is enabled
- **THEN** the window hides and the process continues running

#### Scenario: Window closes and quits when setting is disabled
- **WHEN** the user closes the window and "minimize on close" is disabled
- **THEN** the process exits

---

### Requirement: Open at login
The app SHALL support registering as a login item on macOS so it launches automatically on user login when the setting is enabled.

#### Scenario: Enabling open-at-login registers the login item
- **WHEN** the user enables "Open at login" in settings
- **THEN** the app is registered as a macOS login item via LaunchAgent

#### Scenario: Disabling open-at-login removes the login item
- **WHEN** the user disables "Open at login" in settings
- **THEN** the macOS login item registration is removed

---

### Requirement: App icon and identity metadata
The app SHALL have a configured bundle identifier, display name, and icon that are easy to replace.

#### Scenario: App metadata is set in tauri.conf.json
- **WHEN** the app is built
- **THEN** the bundle identifier is `com.chatwoot.desktop`, the display name is `Chatwoot`, and the icon is sourced from `icons/` in the project

#### Scenario: Icon replacement is documented
- **WHEN** a developer wants to change the app icon
- **THEN** replacing files in the `icons/` directory and running `tauri icon` regenerates all required icon sizes
