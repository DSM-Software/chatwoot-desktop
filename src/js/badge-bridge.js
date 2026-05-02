/**
 * badge-bridge.js
 *
 * Injected into the Chatwoot WebView after page load.
 * Polls the DOM every 5 seconds for the unread conversation count and
 * updates the macOS dock badge via the set_badge_count Tauri command.
 *
 * Failures are silent — the badge just won't update.
 */
;(function () {
  'use strict'

  let lastCount = -1

  // Possible selectors for Chatwoot's unread badge element
  const BADGE_SELECTORS = [
    '[data-key="conversations-badge"]',
    '.conversations-badge',
    '.badge-count',
    '.unread-count',
  ]

  function getUnreadCount() {
    for (const selector of BADGE_SELECTORS) {
      const el = document.querySelector(selector)
      if (el) {
        const text = el.textContent?.trim()
        const num = parseInt(text, 10)
        if (!isNaN(num)) return num
      }
    }
    return 0
  }

  function updateBadge() {
    try {
      const count = getUnreadCount()
      if (count !== lastCount) {
        lastCount = count
        window.__TAURI__?.core?.invoke('set_badge_count', { count }).catch(() => {})
      }
    } catch {
      // Silent failure
    }
  }

  // Start polling
  setInterval(updateBadge, 5000)

  // Also update on DOM mutations (faster response)
  try {
    const observer = new MutationObserver(updateBadge)
    observer.observe(document.body, { childList: true, subtree: true, characterData: true })
  } catch {
    // Fall back to polling only
  }
})()
