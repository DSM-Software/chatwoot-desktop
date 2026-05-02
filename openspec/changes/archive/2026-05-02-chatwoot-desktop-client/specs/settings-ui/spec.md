## ADDED Requirements

### Requirement: Settings screen layout and access
The app SHALL provide a settings screen accessible from the native menu (⌘,) and from the error screen.

#### Scenario: Settings screen opens via keyboard shortcut
- **WHEN** the user presses ⌘,
- **THEN** the settings screen overlays or replaces the current view

#### Scenario: Settings screen can be closed without saving
- **WHEN** the user clicks Cancel or presses Escape on the settings screen
- **THEN** the settings screen is dismissed and all unsaved changes are discarded

---

### Requirement: Workspace URL setting
The settings screen SHALL display the current workspace URL in an editable text field.

#### Scenario: Current URL is pre-filled in the field
- **WHEN** the settings screen opens
- **THEN** the workspace URL field shows the currently saved URL

#### Scenario: Saving a new URL validates and persists it
- **WHEN** the user edits the URL and clicks Save
- **THEN** the new URL is validated, saved to the store, and the WebView reloads with the new URL

#### Scenario: Invalid URL shows inline error and blocks save
- **WHEN** the user enters an invalid URL and clicks Save
- **THEN** an error message is shown beneath the field and the save is blocked

---

### Requirement: Open-at-login toggle
The settings screen SHALL include a toggle for "Open at login".

#### Scenario: Toggle reflects current autostart state
- **WHEN** the settings screen opens
- **THEN** the toggle reflects whether the app is currently registered as a login item

#### Scenario: Enabling toggle registers login item
- **WHEN** the user enables the toggle and saves
- **THEN** the app registers itself as a macOS login item

#### Scenario: Disabling toggle removes login item
- **WHEN** the user disables the toggle and saves
- **THEN** the macOS login item registration is removed

---

### Requirement: Minimize-on-close toggle
The settings screen SHALL include a toggle for "Keep running when window is closed".

#### Scenario: Toggle reflects current minimize-on-close state
- **WHEN** the settings screen opens
- **THEN** the toggle reflects the stored value of the minimize-on-close preference

#### Scenario: Preference is persisted on save
- **WHEN** the user changes the toggle and saves
- **THEN** the new preference is written to the config store and takes effect immediately

---

### Requirement: Notifications toggle
The settings screen SHALL include a toggle for enabling or disabling desktop notifications.

#### Scenario: Toggle reflects current notification preference
- **WHEN** the settings screen opens
- **THEN** the toggle reflects the stored notification preference

#### Scenario: Disabling notifications suppresses JS proxy
- **WHEN** the user disables notifications and saves
- **THEN** the WebView's JS notification interceptor stops forwarding events to the native layer

---

### Requirement: Clear session action
The settings screen SHALL include a "Clear session and log out" button that resets the WebView session.

#### Scenario: Clear session prompts confirmation
- **WHEN** the user clicks "Clear session"
- **THEN** a confirmation dialog is shown warning that the user will be logged out

#### Scenario: Confirmed clear session wipes WebView data and reloads
- **WHEN** the user confirms the clear session action
- **THEN** WebView cookies and localStorage are cleared and the WebView reloads the workspace URL, showing the Chatwoot login screen

#### Scenario: Cancelled clear session has no effect
- **WHEN** the user clicks "Clear session" but cancels the confirmation dialog
- **THEN** no data is cleared and the settings screen remains open
