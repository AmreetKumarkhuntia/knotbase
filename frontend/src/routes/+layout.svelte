<script lang="ts">
  import type { Snippet } from 'svelte'
  import TopBar from '$lib/components/TopBar.svelte'
  import { shell } from '$lib/stores/shell.svelte'
  import { bus } from '$lib/events/bus'
  import { theme } from '$lib/stores/theme.svelte'
  import { notify } from '$lib/stores/notify.svelte'

  // The one navbar for every page; pages describe theirs through the shell store.
  let { children }: { children: Snippet } = $props()
</script>

<TopBar
  crumbs={shell.crumbs}
  stats={shell.stats}
  action={shell.action}
  dark={theme.name === 'console'}
  onToggleTheme={() => bus.emit('theme:toggle', {})}
  error={notify.error}
/>
{@render children()}
