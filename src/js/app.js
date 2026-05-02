import { invoke, listen } from './api.js'
import { navigate } from './router.js'

const loadingOverlay = document.getElementById('loading-overlay')

function showLoading() {
  loadingOverlay?.classList.remove('hidden')
}

function hideLoading() {
  loadingOverlay?.classList.add('hidden')
}

/** Navigate the WebView to the workspace URL. */
async function loadWorkspace(url) {
  showLoading()
  try {
    await invoke('navigate_workspace', { url })
  } catch (err) {
    console.error('[app] navigate_workspace failed:', err)
    hideLoading()
    navigate('#view-error')
    const detail = document.getElementById('error-detail')
    if (detail) detail.textContent = String(err)
  }
}

// Listen for Rust events about WebView load state.
// These fire when the WebView (navigated to workspace) reports status back.
async function setupEventListeners() {
  await listen('webview:loaded', () => {
    hideLoading()
  })

  await listen('webview:error', (event) => {
    hideLoading()
    navigate('#view-error')
    const detail = document.getElementById('error-detail')
    if (detail) {
      detail.textContent = event.payload?.message || 'Unable to reach workspace.'
    }
  })

  // Reload menu event
  await listen('menu:reload', () => {
    if (window.__currentView === '#view-app') {
      showLoading()
      invoke('navigate_workspace', {
        url: window.__lastWorkspaceUrl || '',
      }).catch(() => {})
    }
  })

  // Settings menu event
  await listen('menu:settings', () => {
    navigate('#view-settings')
    window.__settingsModule?.init()
  })
}

// Expose loadWorkspace for setup.js and other modules
window.__appModule = { loadWorkspace, showLoading, hideLoading }

// Listen for notification click events — bring window to front (task 10.5)
async function setupNotificationListener() {
  await listen('notification-action-performed', () => {
    window.__TAURI__?.window?.getCurrentWindow?.()?.show?.()
    window.__TAURI__?.window?.getCurrentWindow?.()?.setFocus?.()
  })
}

// Set up event listeners when the module loads
setupEventListeners()
setupNotificationListener()
