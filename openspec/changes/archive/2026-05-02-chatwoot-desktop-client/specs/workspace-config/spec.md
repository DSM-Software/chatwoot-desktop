## ADDED Requirements

### Requirement: First-run setup screen
On first launch (no workspace URL stored), the app SHALL display a setup screen prompting the user to enter their Chatwoot workspace URL before the main WebView loads.

#### Scenario: Setup screen appears on first launch
- **WHEN** the app launches and no workspace URL is stored in the local config
- **THEN** the setup screen is shown instead of the WebView

#### Scenario: Setup screen is skipped on subsequent launches
- **WHEN** the app launches and a workspace URL is already stored
- **THEN** the WebView loads directly with the stored URL

---

### Requirement: Workspace URL validation
The app SHALL validate the workspace URL before saving it.

#### Scenario: Valid HTTPS URL is accepted
- **WHEN** the user enters a syntactically valid URL starting with `https://`
- **THEN** the URL is accepted and saved

#### Scenario: Valid HTTP URL is accepted (self-hosted without TLS)
- **WHEN** the user enters a syntactically valid URL starting with `http://`
- **THEN** the URL is accepted with a warning that the connection is not encrypted

#### Scenario: Invalid URL shows inline error
- **WHEN** the user submits a URL that is empty, malformed, or missing a scheme
- **THEN** an inline error message is shown and the URL is not saved

#### Scenario: Trailing slashes are normalized
- **WHEN** the user enters a URL with a trailing slash (e.g., `https://app.chatwoot.com/`)
- **THEN** the trailing slash is stripped before saving

---

### Requirement: Workspace URL persistence
The workspace URL SHALL be stored in the local app config store and survive app restarts.

#### Scenario: URL is written to store on save
- **WHEN** the user confirms a valid URL in the setup screen or settings
- **THEN** the URL is written to the persistent config store (key: `workspaceUrl`)

#### Scenario: URL is read from store on launch
- **WHEN** the app launches
- **THEN** the stored URL is loaded from the config store before any screen is rendered

---

### Requirement: Workspace URL change from settings
The user SHALL be able to update the workspace URL at any time via the settings screen.

#### Scenario: Changing URL reloads the WebView
- **WHEN** the user saves a new workspace URL from the settings screen
- **THEN** the WebView navigates to the new URL immediately

#### Scenario: Cancelling settings change has no effect
- **WHEN** the user opens settings, edits the URL field, and then cancels
- **THEN** the stored URL is unchanged and the WebView continues showing the previous URL
