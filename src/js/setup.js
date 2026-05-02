import { invoke } from './api.js'
import { navigate } from './router.js'
import { validateUrl } from './validate-url.js'

const form = document.getElementById('setup-form')
const urlInput = document.getElementById('setup-url')
const errorEl = document.getElementById('setup-url-error')
const warningEl = document.getElementById('setup-url-warning')

function showError(msg) {
  errorEl.textContent = msg || ''
  warningEl.textContent = ''
}

function showWarning(msg) {
  warningEl.textContent = msg || ''
}

function clearMessages() {
  errorEl.textContent = ''
  warningEl.textContent = ''
}

async function init() {
  try {
    const config = await invoke('get_config')
    if (config.workspaceUrl) {
      // Already configured — go to app view
      navigate('#view-app')
      // Trigger webview navigation
      window.__appModule?.loadWorkspace(config.workspaceUrl)
      return
    }
  } catch (err) {
    console.warn('[setup] Could not load config:', err)
  }

  // Show setup view
  navigate('#view-setup')
  urlInput.focus()
}

form?.addEventListener('submit', async (e) => {
  e.preventDefault()
  clearMessages()

  const { url, error, warning } = validateUrl(urlInput.value)

  if (error) {
    showError(error)
    return
  }

  if (warning) {
    showWarning(warning)
  }

  try {
    await invoke('cmd_save_config', {
      config: {
        workspaceUrl: url,
        minimizeOnClose: true,
        openAtLogin: false,
        notificationsEnabled: true,
        windowWidth: 1280,
        windowHeight: 800,
        windowX: null,
        windowY: null,
      },
    })
    navigate('#view-app')
    window.__appModule?.loadWorkspace(url)
  } catch (err) {
    showError('Failed to save configuration: ' + err)
  }
})

urlInput?.addEventListener('input', clearMessages)

// Initialise on load
init()
