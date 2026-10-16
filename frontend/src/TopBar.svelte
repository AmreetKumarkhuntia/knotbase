<script lang="ts">
  import Link from './Link.svelte'

  let { crumbs }: { crumbs: { id: number; name: string }[] } = $props()
</script>

<nav class="topbar" aria-label="Breadcrumb">
  <Link href="/" class="wordmark">knotbase</Link>
  {#each crumbs as c, i (c.id)}
    <span class="sep" aria-hidden="true">/</span>
    {#if i === crumbs.length - 1}
      <span aria-current="page">{c.name}</span>
    {:else}
      <Link href="/node/{c.id}">{c.name}</Link>
    {/if}
  {/each}
</nav>
