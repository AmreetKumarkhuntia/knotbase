<script lang="ts">
  import { LevelState, provideLevel } from '$lib/stores/level.svelte'
  import { data } from '$lib/stores/data.svelte'
  import { notify } from '$lib/stores/notify.svelte'
  import { shell } from '$lib/stores/shell.svelte'
  import { plural } from '$lib/utils/format'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'
  import EmptyCard from '$lib/components/EmptyCard.svelte'
  import NodeFlow from '$lib/widgets/NodeFlow.svelte'
  import Inspector from '$lib/widgets/inspector/Inspector.svelte'

  // App keys this page by id, so every level starts with fresh UI state; the data comes from the store (and the cache survives).
  let { id }: { id: number } = $props()
  notify.error = '' // a banner from the previous page is not about this one
  const level = new LevelState(() => id)
  provideLevel(level)
  const failed = $derived(typeof data.status(`node:${id}`) === 'object')
  $effect(() => {
    const g = level.graph
    shell.crumbs = level.node?.breadcrumb ?? []
    shell.stats = g ? `${plural(g.nodes.length, 'node')} · ${plural(g.edges.length, 'connection')}` : ''
    shell.action = {
      label: 'Add node',
      icon: 'plus',
      disabled: !level.node,
      onClick: () => (level.adding = !level.adding),
    }
  })
</script>

<svelte:head><title>{level.node ? `${level.node.name} · knotbase` : 'knotbase'}</title></svelte:head>

{#if level.node && level.graph}
  <main class="split">
    <section class="canvas-col" aria-label="Graph of {level.node.name}">
      <NodeFlow />
      {#if !level.adding && level.graph.nodes.length === 0}
        <div class="cover">
          <EmptyCard
            title="Nothing inside {level.node.name} yet"
            text="Add a part, a flow or a step. Each node gets its own graph, doc and checklist."
          >
            <Button variant="primary" onclick={() => (level.adding = true)}><Icon name="plus" />Add node</Button>
          </EmptyCard>
        </div>
      {/if}
    </section>
    <aside class="inspector" aria-label="Inspector"><Inspector /></aside>
  </main>
{:else if failed}
  <main class="cover static">
    <EmptyCard title="This node can't be loaded" text="It may have been deleted, or the server is down.">
      <Button href="/" variant="primary">Back to Home</Button>
    </EmptyCard>
  </main>
{:else}
  <p class="empty">Loading…</p>
{/if}

<style>
  .split {
    display: flex;
    flex-wrap: wrap;
    flex: 1 1 auto;
    min-height: 0;
  }
  .canvas-col {
    position: relative;
    display: flex;
    flex: 999 1 560px;
    min-width: 0;
    min-height: 600px;
  }
  .inspector {
    display: flex;
    flex-direction: column;
    flex: 1 1 384px;
    min-width: 0;
    overflow-y: auto;
    background: var(--chrome);
    border-left: 1px solid var(--rule);
  }
  .cover {
    position: absolute;
    inset: 0;
    z-index: 4;
    display: grid;
    place-items: center;
    pointer-events: none;
  }
  .cover.static {
    position: relative;
    flex: 1 1 auto;
    pointer-events: auto;
  }
</style>
