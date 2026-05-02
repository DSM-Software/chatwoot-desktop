## MODIFIED Requirements

### Requirement: Notifications toggle
The settings screen SHALL include a toggle for enabling or disabling desktop notifications that takes effect immediately upon saving, without requiring a WebView reload.

#### Scenario: Toggle reflects current notification preference
- **WHEN** the settings screen opens
- **THEN** the toggle reflects the stored notification preference

#### Scenario: Disabling notifications suppresses JS proxy immediately
- **WHEN** the user disables notifications and saves
- **THEN** the WebView's JS notification interceptor stops forwarding events to the native layer immediately in the current session

#### Scenario: Enabling notifications restores JS proxy immediately
- **WHEN** the user enables notifications and saves
- **THEN** the WebView's JS notification interceptor resumes forwarding events to the native layer immediately in the current session
