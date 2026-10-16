<script lang="ts">
  import { nav } from './nav.svelte'
  import NodePage from './NodePage.svelte'

  // shortcut: one route; switch to a router lib when nested layouts or more than ~3 routes appear.
  const id = $derived(Number(nav.path.match(/^\/node\/(\d+)\/?$/)?.[1] ?? 1))
</script>

<NodePage {id} />
