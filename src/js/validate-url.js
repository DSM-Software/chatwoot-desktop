/**
 * Shared URL validation logic used by setup.js and settings.js.
 *
 * Returns: { url: string, error: string|null, warning: string|null }
 */
export function validateUrl(raw) {
  const trimmed = raw.trim()

  if (!trimmed) {
    return { url: null, error: 'Please enter a workspace URL.', warning: null }
  }

  let parsed
  try {
    parsed = new URL(trimmed)
  } catch {
    return { url: null, error: 'Invalid URL. Enter a URL like https://app.chatwoot.com', warning: null }
  }

  if (parsed.protocol !== 'http:' && parsed.protocol !== 'https:') {
    return { url: null, error: 'URL must start with http:// or https://', warning: null }
  }

  if (!parsed.hostname) {
    return { url: null, error: 'URL must have a valid hostname.', warning: null }
  }

  // Normalise: strip trailing slash from pathname
  let normalized = parsed.toString()
  if (normalized.endsWith('/') && parsed.pathname === '/') {
    normalized = normalized.slice(0, -1)
  }

  const warning =
    parsed.protocol === 'http:'
      ? 'Warning: this connection is not encrypted (HTTP). Use HTTPS if possible.'
      : null

  return { url: normalized, error: null, warning }
}
