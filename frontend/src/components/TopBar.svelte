<script lang="ts">
  import type { Snippet } from 'svelte'
  import Link from './Link.svelte'
  import Swatch from './Swatch.svelte'
  import { ui } from '../ui.svelte'

  // crumbs come from the API and start at root 1, which renders as "Home"; kind is optional until the backend sends it
  let { crumbs = [], children }: { crumbs?: { id: number; name: string; kind?: string }[]; children?: Snippet } = $props()
</script>

<header class="topbar">
  <Link href="/" class="wordmark">knotbase</Link>
  <nav class="crumbs" aria-label="Breadcrumb">
    <span class="sep" aria-hidden="true">/</span>
    {#if crumbs.length === 0}<span aria-current="page">Home</span>{:else}<Link href="/">Home</Link>{/if}
    {#each crumbs as c, i (c.id)}
      {#if c.id !== 1}
        <span class="sep" aria-hidden="true">/</span>
        {#if i === crumbs.length - 1}
          <span aria-current="page">{#if c.kind}<Swatch kind={c.kind} />{/if}{c.name}</span>
        {:else}
          <Link href="/node/{c.id}">{#if c.kind}<Swatch kind={c.kind} />{/if}{c.name}</Link>
        {/if}
      {/if}
    {/each}
  </nav>
  <span class="spacer"></span>
  {@render children?.()}
</header>
{#if ui.error}<p class="error" role="alert">{ui.error}</p>{/if}
