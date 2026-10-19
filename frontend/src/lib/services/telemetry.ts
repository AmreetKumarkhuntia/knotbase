import { bus } from '$lib/events/bus'

// Every event passes through here once, after its handler ran: name, payload, result or error, duration.
// shortcut: console only; batch to an endpoint when there is one.
export function register() {
  bus.tap(({ name, payload, result, error, ms }) => {
    if (import.meta.env.DEV)
      console.debug(`[event] ${name} ${error ? `failed: ${error}` : 'ok'} ${ms.toFixed(0)}ms`, payload, result ?? '')
  })
}
