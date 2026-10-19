<script lang="ts">
  import SystemGrid from '$lib/widgets/SystemGrid.svelte'
  import { shell } from '$lib/stores/shell.svelte'
  import { notify } from '$lib/stores/notify.svelte'
  import { plural } from '$lib/utils/format'

  let adding = $state(false)
  let count = $state<number>()
  notify.error = '' // a banner from the previous page is not about this one
  $effect(() => {
    shell.crumbs = undefined
    shell.stats = count == null ? '' : plural(count, 'system')
    shell.action = { label: 'New system', icon: 'plus', onClick: () => (adding = !adding) }
  })
</script>

<svelte:head><title>knotbase</title></svelte:head>

<main class="home">
  <section class="systems" aria-labelledby="systems-h">
    <div class="stack tight">
      <h1 id="systems-h">Systems</h1>
      <p class="muted">Independent maps. Each one has its own graph, docs and items.</p>
    </div>
    <SystemGrid bind:adding onLoad={(s) => (count = s.length)} />
  </section>
</main>

<style>
  .home {
    display: flex;
    flex-wrap: wrap;
    gap: 32px;
    flex: 1 1 auto;
    width: 100%;
    max-width: 1280px;
    margin: 0 auto;
    padding: 32px 24px 48px;
    overflow-y: auto;
  }
  .systems {
    display: flex;
    flex-direction: column;
    gap: 16px;
    flex: 999 1 560px;
    min-width: 0;
  }
  .systems h1 {
    font-size: 24px;
    line-height: 30px;
  }
</style>
