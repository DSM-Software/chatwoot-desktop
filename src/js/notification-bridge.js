/**
 * notification-bridge.js
 *
 * Injected into the Chatwoot WebView after page load.
 * Overrides window.Notification to forward calls to native macOS notifications
 * via the Tauri send_notification command.
 *
 * Only active when notificationsEnabled is true in app config.
 */
;(function () {
  'use strict'

  const OriginalNotification = window.Notification

  function TauriNotification(title, options) {
    // Respect the runtime toggle synchronously via the injected config shim
    if (!window.__cwDesktop?.notificationsEnabled) {
      return new OriginalNotification(title, options)
    }

    // Call send_notification Tauri command
    try {
      const body = options?.body || ''
      window.__TAURI__?.core?.invoke('send_notification', { title, body }).catch(() => {})
    } catch (e) {
      // Silently fall back to original
      return new OriginalNotification(title, options)
    }

    // Return a minimal Notification-like object
    const handlers = {}
    return {
      close() {},
      addEventListener(type, fn) { handlers[type] = fn },
      removeEventListener(type) { delete handlers[type] },
      get onclick() { return handlers.click || null },
      set onclick(fn) { handlers.click = fn },
    }
  }

  // Copy static properties
  TauriNotification.requestPermission = async () => {
    try {
      return await window.__TAURI__?.core?.invoke('request_notification_permission') || 'granted'
    } catch {
      return 'granted'
    }
  }

  TauriNotification.permission = 'granted'

  Object.defineProperty(window, 'Notification', {
    value: TauriNotification,
    writable: true,
    configurable: true,
  })
})()
