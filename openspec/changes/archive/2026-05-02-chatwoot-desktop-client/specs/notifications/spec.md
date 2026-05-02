## ADDED Requirements

### Requirement: Native macOS notifications
The app SHALL deliver Chatwoot notifications as native macOS notifications using `tauri-plugin-notification`.

#### Scenario: Chatwoot notification is forwarded to macOS
- **WHEN** Chatwoot's web code calls the browser `Notification` constructor
- **THEN** the app intercepts the call via JS injection and dispatches a native macOS notification with the same title and body

#### Scenario: Notifications respect the user preference
- **WHEN** the user has disabled notifications in settings
- **THEN** the JS injection does not forward notifications to the native layer

---

### Requirement: Click-to-focus on notification
Clicking a native desktop notification SHALL bring the main app window to the foreground.

#### Scenario: Clicking notification focuses the window
- **WHEN** the user clicks a native notification
- **THEN** the main window becomes visible and gains system focus

#### Scenario: Clicking notification works when window is hidden in tray
- **WHEN** the main window is hidden and the user clicks a notification
- **THEN** the window is shown and brought to the foreground

---

### Requirement: Notification permission request
The app SHALL request macOS notification permission on first launch (or first notification event) if permission has not been granted.

#### Scenario: Permission is requested before first notification
- **WHEN** the first Chatwoot notification event is intercepted and permission status is undetermined
- **THEN** the macOS permission dialog is shown to the user

#### Scenario: Notifications are suppressed when permission is denied
- **WHEN** the user has denied notification permission in macOS settings
- **THEN** no notification is shown and no error is thrown

---

### Requirement: App dock badge for unread count (best-effort)
The app SHALL attempt to display an unread conversation count as a badge on the macOS dock icon.

#### Scenario: Badge is updated when unread count changes
- **WHEN** the injected JS detects a change in Chatwoot's unread count (via DOM polling or event)
- **THEN** the Tauri backend updates the dock badge with the new count

#### Scenario: Badge is cleared when all conversations are read
- **WHEN** the unread count returns to zero
- **THEN** the dock badge is removed

#### Scenario: Badge update failure is silent
- **WHEN** the DOM element used for unread count detection cannot be found
- **THEN** no error is shown to the user and the badge retains its last known value or is absent
