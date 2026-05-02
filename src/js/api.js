/**
 * Thin wrapper around Tauri's IPC invoke.
 * Uses window.__TAURI__.core.invoke which is injected by Tauri.
 */

function getInvoke() {
  return window?.__TAURI__?.core?.invoke
}

function getEvent() {
  return window?.__TAURI__?.event
}

export async function invoke(command, args = {}) {
  const fn = getInvoke()
  if (!fn) {
    console.warn('[api] Tauri invoke not available — running outside Tauri?')
    throw new Error('Tauri IPC not available')
  }
  return fn(command, args)
}

export async function listen(event, handler) {
  const eventApi = getEvent()
  if (!eventApi) {
    console.warn('[api] Tauri event API not available')
    return () => {}
  }
  return eventApi.listen(event, handler)
}

export async function emit(event, payload) {
  const eventApi = getEvent()
  if (!eventApi) return
  return eventApi.emit(event, payload)
}

// Make available globally for non-module scripts (bridge injections)
window.__cw_api = { invoke, listen, emit }
