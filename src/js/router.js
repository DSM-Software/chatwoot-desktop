/**
 * Simple view router — shows/hides views by ID.
 * Views are elements with the `.view` class; active view gets `.active`.
 */

const VIEWS = ['#view-setup', '#view-app', '#view-error', '#view-settings']

function navigate(viewId) {
  const normalised = viewId.startsWith('#') ? viewId : '#' + viewId

  VIEWS.forEach((id) => {
    const el = document.querySelector(id)
    if (!el) return
    if (id === normalised) {
      el.classList.add('active')
    } else {
      el.classList.remove('active')
    }
  })

  // Store current view so other modules can query it
  window.__currentView = normalised
}

function currentView() {
  return window.__currentView || null
}

// Expose globally for use by Rust-injected scripts and inline handlers
window.__router = { navigate, currentView }

export { navigate, currentView }
