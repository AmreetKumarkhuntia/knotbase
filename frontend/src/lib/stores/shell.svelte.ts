import type { TopBarAction } from '$lib/components/TopBar.svelte'

// What the current page shows in the navbar. The layout renders it; every page sets all three fields in an effect.
export const shell = $state<{ crumbs?: { id: number; name: string }[]; stats: string; action?: TopBarAction }>({
  stats: '',
})
