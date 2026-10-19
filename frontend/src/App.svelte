<script lang="ts">
  import { nav, navigate } from './nav.svelte'
  import HomePage from './pages/HomePage.svelte'
  import NodePage from './pages/NodePage.svelte'

  // shortcut: two routes by regex; switch to a router lib when nested layouts or more than ~3 routes appear.
  const id = $derived(Number(nav.path.match(/^\/node\/(\d+)\/?$/)?.[1] ?? 0))
  $effect(() => {
    if (id === 1) navigate('/', true)
  }) // root 1 is the hidden Home container; replace, so Back doesn't bounce
</script>

{#if id > 1}
  {#key id}<NodePage {id} />{/key}
{:else}
  <HomePage />
{/if}
