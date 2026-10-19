import { bus } from '$lib/events/bus'
import { navigate } from '$lib/stores/router.svelte'

// Every navigation is an event handled here; nothing else calls navigate(). Root 1 is the hidden Home container.
export function register() {
  bus.on('nav:to', async ({ path, replace }) => navigate(path, replace))
  bus.on('nav:home', async () => navigate('/'))
  bus.on('nav:node', async ({ id }) => navigate(id == null || id === 1 ? '/' : `/node/${id}`))
}
