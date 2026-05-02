import { invoke } from './api.js'
import { navigate } from './router.js'

const btnRetry = document.getElementById('btn-retry')
const btnChangeUrl = document.getElementById('btn-change-url')

btnRetry?.addEventListener('click', async () => {
  try {
    const config = await invoke('get_config')
    if (config.workspaceUrl) {
      navigate('#view-app')
      window.__appModule?.loadWorkspace(config.workspaceUrl)
    } else {
      navigate('#view-setup')
    }
  } catch (err) {
    console.error('[error] retry failed:', err)
  }
})

btnChangeUrl?.addEventListener('click', () => {
  navigate('#view-settings')
  window.__settingsModule?.init()
})
