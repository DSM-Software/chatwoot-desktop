## MODIFIED Requirements

### Requirement: Native desktop notifications
The app SHALL deliver Chatwoot notifications as native desktop notifications using `tauri-plugin-notification` on both macOS and Windows.

#### Scenario: Chatwoot notification is forwarded to the OS on macOS
- **WHEN** Chatwoot's web code calls the browser `Notification` constructor on macOS
- **THEN** the app intercepts the call via JS injection and dispatches a native macOS notification with the same title and body

#### Scenario: Chatwoot notification is forwarded to the OS on Windows
- **WHEN** Chatwoot's web code calls the browser `Notification` constructor on Windows
- **THEN** the app intercepts the call via JS injection and dispatches a native Windows Toast notification with the same title and body

#### Scenario: Notifications respect the user preference in real time
- **WHEN** the user has disabled notifications in settings and saves
- **THEN** the JS injection immediately stops forwarding notifications to the native layer without requiring a WebView reload

#### Scenario: Notifications are re-enabled in real time
- **WHEN** the user re-enables notifications in settings and saves
- **THEN** the JS injection immediately resumes forwarding notifications to the native layer without requiring a WebView reload

---

### Requirement: Notification permission request
The app SHALL request notification permission on first launch or first notification event if permission has not been granted.

#### Scenario: Permission is requested before first notification on macOS
- **WHEN** the first Chatwoot notification event is intercepted on macOS and permission status is undetermined
- **THEN** the macOS permission dialog is shown to the user

#### Scenario: Permission is always considered granted on Windows
- **WHEN** `request_notification_permission` is called on Windows
- **THEN** the command returns `"granted"` immediately without showing any system dialog (Windows manages notification access via OS settings)

#### Scenario: Notifications are suppressed when OS permission is denied
- **WHEN** the user has denied notification permission in system settings (macOS or Windows)
- **THEN** no notification is shown and no error is thrown
