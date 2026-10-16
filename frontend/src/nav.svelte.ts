// An object, not a bare $state: exported state can't be reassigned across modules.
export const nav = $state({ path: location.pathname })

addEventListener('popstate', () => (nav.path = location.pathname))

export function navigate(to: string) {
  if (to !== location.pathname) history.pushState(null, '', to)
  nav.path = to
}
