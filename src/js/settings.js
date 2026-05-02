import { invoke, listen } from './api.js'
import { navigate } from './router.js'
import { validateUrl } from './validate-url.js'

const urlInput = document.getElementById('settings-url')
const urlError = document.getElementById('settings-url-error')
const urlWarning = document.getElementById('settings-url-warning')
const toggleOpenAtLogin = document.getElementById('settings-open-at-login')
const toggleMinimizeOnClose = document.getElementById('settings-minimize-on-close')
const toggleNotifications = document.getElementById('settings-notifications')
const btnSave = document.getElementById('btn-settings-save')
const btnCancel = document.getElementById('btn-settings-cancel')
const btnClearSession = document.getElementById('btn-clear-session')

let originalUrl = ''

async function init() {
  urlError.textContent = ''
  urlWarning.textContent = ''

  try {
    const config = await invoke('get_config')
    originalUrl = config.workspaceUrl || ''
    urlInput.value = originalUrl
    toggleOpenAtLogin.checked = !!config.openAtLogin
    toggleMinimizeOnClose.checked = config.minimizeOnClose !== false
    toggleNotifications.checked = config.notificationsEnabled !== false
  } catch (err) {
    console.error('[settings] failed to load config:', err)
  }
}

urlInput?.addEventListener('input', () => {
  urlError.textContent = ''
  urlWarning.textContent = ''
})

btnCancel?.addEventListener('click', () => {
  // Discard changes and go back
  const prevView = window.__prevView || '#view-app'
  navigate(prevView)
})

btnSave?.addEventListener('click', async () => {
  urlError.textContent = ''
  urlWarning.textContent = ''

  const { url, error, warning } = validateUrl(urlInput.value)

  if (error) {
    urlError.textContent = error
    return
  }

  if (warning) {
    urlWarning.textContent = warning
  }

  try {
    const currentConfig = await invoke('get_config')
    const newConfig = {
      ...currentConfig,
      workspaceUrl: url,
      openAtLogin: toggleOpenAtLogin.checked,
      minimizeOnClose: toggleMinimizeOnClose.checked,
      notificationsEnabled: toggleNotifications.checked,
    }

    await invoke('cmd_save_config', { config: newConfig })

    // Apply autostart change via plugin
    try {
      if (newConfig.openAtLogin) {
        await invoke('plugin:autostart|enable')
      } else {
        await invoke('plugin:autostart|disable')
      }
    } catch (autostartErr) {
      console.warn('[settings] autostart change failed:', autostartErr)
    }

    const urlChanged = url !== originalUrl

    navigate('#view-app')

    if (urlChanged) {
      // Navigate WebView to new URL
      window.__appModule?.loadWorkspace(url)
    }
  } catch (err) {
    urlError.textContent = 'Failed to save settings: ' + err
  }
})

btnClearSession?.addEventListener('click', async () => {
  const confirmed = await window.__TAURI__?.dialog?.ask(
    'This will clear your session and sign you out of Chatwoot. Continue?',
    { title: 'Clear Session', kind: 'warning' }
  )

  if (!confirmed) return

  try {
    await invoke('clear_session')
    navigate('#view-app')
    window.__appModule?.showLoading()
  } catch (err) {
    urlError.textContent = 'Failed to clear session: ' + err
  }
})

// Wire ⌘, / menu:settings Tauri event → navigate to this view
listen('menu:settings', () => {
  window.__prevView = window.__currentView || '#view-app'
  navigate('#view-settings')
  init()
}).catch(() => {})

// Expose init for use by other modules
window.__settingsModule = { init }

// Auto-initialise if settings view is shown on load (e.g. via hash)
if (window.location.hash === '#view-settings') {
  init()
}
