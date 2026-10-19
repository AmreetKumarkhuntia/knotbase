import { bus } from '$lib/events/bus'

// An object, not a bare $state: exported state can't be reassigned across modules.
export const router = $state({ path: location.pathname })

addEventListener('popstate', () => (router.path = location.pathname))

// Called by the nav handler only (lib/services/nav.ts); everything else emits a nav:* event.
export function navigate(to: string, replace = false) {
  if (to !== location.pathname) history[replace ? 'replaceState' : 'pushState'](null, '', to)
  router.path = to
}

// Same-origin links become nav:to events; modified clicks, downloads, target= and external hrefs keep the browser's own behaviour.
document.addEventListener('click', (e) => {
  if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return
  const a = (e.target as Element).closest<HTMLAnchorElement>('a[href]')
  if (!a || a.target || a.hasAttribute('download') || a.origin !== location.origin) return
  e.preventDefault()
  bus.emit('nav:to', { path: a.pathname + a.search + a.hash })
})
