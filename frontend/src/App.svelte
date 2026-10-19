<script lang="ts">
  import { bus } from '$lib/events/bus'
  import { router } from '$lib/stores/router.svelte'
  import Layout from './routes/+layout.svelte'
  import Home from './routes/+page.svelte'
  import Node from './routes/node/[id]/+page.svelte'

  // shortcut: two routes by regex; switch to a router lib when nested layouts or more than ~3 routes appear.
  const id = $derived(Number(router.path.match(/^\/node\/(\d+)\/?$/)?.[1] ?? 0))
  $effect(() => {
    if (id === 1) bus.emit('nav:to', { path: '/', replace: true }) // root 1 is the hidden Home container; replace, so Back doesn't bounce
  })
</script>

<Layout>
  {#if id > 1}
    {#key id}<Node {id} />{/key}
  {:else}
    <Home />
  {/if}
</Layout>
