<script lang="ts">
  import type { Snippet } from 'svelte'
  import type { HTMLAnchorAttributes } from 'svelte/elements'
  import { navigate } from './nav.svelte'

  let { href, children, ...rest }: { href: string; children: Snippet } & HTMLAnchorAttributes = $props()

  function click(e: MouseEvent) {
    // modified clicks keep the browser's own behaviour (new tab, copy link)
    if (e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return
    e.preventDefault()
    navigate(href)
  }
</script>

<a {href} {...rest} onclick={click}>{@render children()}</a>
