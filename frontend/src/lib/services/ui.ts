import { bus } from '$lib/events/bus'
import { toggleTheme } from '$lib/stores/theme.svelte'

// UI-wide actions that are not data and not navigation.
export function register() {
  bus.on('theme:toggle', async () => toggleTheme())
}
