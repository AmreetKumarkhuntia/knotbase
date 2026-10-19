<script lang="ts">
  import type { Snippet } from 'svelte'
  import Link from './Link.svelte'
  import { ui } from '../ui.svelte'

  // crumbs come from the API and start at root 1, which renders as "Home"; no crumbs prop at all means this is the Home page
  let { crumbs, children }: { crumbs?: { id: number; name: string }[]; children?: Snippet } = $props()
</script>

<header class="topbar">
  <Link href="/" class="wordmark">knotbase</Link>
  <nav class="crumbs" aria-label="Breadcrumb">
    <span class="sep" aria-hidden="true">/</span>
    {#if crumbs}
      <Link href="/">Home</Link>
      {#each crumbs as c, i (c.id)}
        {#if c.id !== 1}
          <span class="sep" aria-hidden="true">/</span>
          {#if i === crumbs.length - 1}<span aria-current="page">{c.name}</span>{:else}<Link href="/node/{c.id}"
              >{c.name}</Link
            >{/if}
        {/if}
      {/each}
    {:else}
      <span aria-current="page">Home</span>
    {/if}
  </nav>
  <span class="spacer"></span>
  {@render children?.()}
</header>
{#if ui.error}<p class="error" role="alert">{ui.error}</p>{/if}
