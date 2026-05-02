## ADDED Requirements

### Requirement: WebView session persistence
The WebView SHALL persist cookies, localStorage, and IndexedDB between app restarts so the user does not need to log in again.

#### Scenario: Session is restored on restart
- **WHEN** the user authenticates in Chatwoot and then restarts the app
- **THEN** the WebView loads with the existing session cookies and the user is already logged in

#### Scenario: Session is cleared on demand
- **WHEN** the user triggers "Clear session" from the settings screen
- **THEN** the WebView data directory is cleared and the user is returned to the Chatwoot login screen

---

### Requirement: Loading state during WebView navigation
The app SHALL display a loading screen while the initial Chatwoot URL is loading.

#### Scenario: Loading screen appears before page finishes loading
- **WHEN** the WebView begins loading the workspace URL
- **THEN** a loading screen is shown overlaid on or replacing the WebView area

#### Scenario: Loading screen is dismissed on page load success
- **WHEN** the WebView emits a page-loaded event
- **THEN** the loading screen is hidden and the WebView becomes visible

---

### Requirement: Error screen on load failure
The app SHALL display a user-friendly error screen when the workspace URL is unreachable or the page fails to load.

#### Scenario: Error screen appears on navigation failure
- **WHEN** the WebView fails to load the workspace URL (network error, DNS failure, timeout)
- **THEN** the loading screen is replaced by an error screen with a description of the problem

#### Scenario: Retry button re-attempts the load
- **WHEN** the user clicks "Try Again" on the error screen
- **THEN** the WebView attempts to reload the workspace URL and the loading screen is shown again

#### Scenario: Error screen provides access to settings
- **WHEN** the error screen is displayed
- **THEN** a "Change URL" button is visible that opens the settings screen

---

### Requirement: External link interception
Any link whose hostname differs from the configured workspace URL's hostname SHALL be opened in the system default browser rather than inside the WebView.

#### Scenario: External link opens in system browser
- **WHEN** a navigation event targets a URL with a different hostname than the workspace URL
- **THEN** the navigation is cancelled in the WebView and `shell.open()` is called with the external URL

#### Scenario: Internal navigation stays in WebView
- **WHEN** a navigation event targets a URL on the same hostname as the workspace URL
- **THEN** the navigation proceeds normally within the WebView

#### Scenario: `target="_blank"` links from Chatwoot are intercepted
- **WHEN** a Chatwoot page triggers `window.open()` or a link with `target="_blank"`
- **THEN** the new window request is intercepted and the URL is opened externally if it is a different domain

---

### Requirement: Navigation domain allowlist
The WebView SHALL only navigate to the configured workspace domain. Navigation to unrelated domains SHALL be blocked or redirected to the system browser.

#### Scenario: Allowlist updated when workspace URL changes
- **WHEN** the user changes the workspace URL in settings
- **THEN** the allowed navigation domain is updated to the new hostname

#### Scenario: Blocked navigation does not crash the app
- **WHEN** a navigation event is blocked
- **THEN** no error dialog appears; the blocked URL is silently redirected to the system browser
