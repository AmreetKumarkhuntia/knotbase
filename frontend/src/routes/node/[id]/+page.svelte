<script lang="ts">
  import { tick } from 'svelte'
  import { api, type Graph, type NodeDetail } from '$lib/services/api'
  import { navigate } from '$lib/stores/router.svelte'
  import { act, load, notify } from '$lib/stores/notify.svelte'
  import { shell } from '$lib/stores/shell.svelte'
  import { plural } from '$lib/utils/format'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'
  import EmptyCard from '$lib/components/EmptyCard.svelte'
  import AddNodeForm from '$lib/components/AddNodeForm.svelte'
  import NodeFlow from '$lib/widgets/NodeFlow.svelte'
  import Inspector from '$lib/widgets/inspector/Inspector.svelte'
  import EdgeInspector from '$lib/widgets/inspector/EdgeInspector.svelte'

  // App keys this page by id, so every level starts with fresh state.
  let { id }: { id: number } = $props()
  let data = $state.raw<{ node: NodeDetail; graph: Graph }>() // raw: never mutated in place, always replaced
  let selectedId = $state<number | null>(null)
  let selectedEdgeId = $state<number | null>(null)
  let adding = $state(false)
  let canvas = $state<ReturnType<typeof NodeFlow>>()
  const edge = $derived(data?.graph.edges.find((e) => e.id === selectedEdgeId))

  let seq = 0 // only the latest reload may write: multi-node drags and inspector edits fire overlapping reloads
  async function reload() {
    const n = ++seq
    await load(
      Promise.all([api.node(id), api.graph(id)]).then(([node, graph]) => {
        if (n === seq) data = { node, graph }
      }),
    )
  }
  notify.error = '' // a banner from the previous page is not about this one
  reload()
  $effect(() => {
    shell.crumbs = data?.node.breadcrumb ?? []
    shell.stats = data
      ? `${plural(data.graph.nodes.length, 'node')} · ${plural(data.graph.edges.length, 'connection')}`
      : ''
    shell.action = { label: 'Add node', icon: 'plus', disabled: !data, onClick: () => (adding = !adding) }
  })
  // shortcut: refetch after every mutation, no optimistic state (a failed one still refetches so the UI snaps back, and the banner stays); revisit if the backend is ever remote.
  async function change(p: Promise<unknown>) {
    const ok = await act(p)
    await reload()
    return ok
  }

  async function addNode(name: string, kind: string) {
    const n = data?.graph.nodes.length ?? 0
    let created: number | null = null
    // new cards fill a 3-wide grid, as the design does
    const ok = await change(
      api
        .createNode({ parent_id: id, name, kind, x: (n % 3) * 300, y: 100 + Math.floor(n / 3) * 200 })
        .then((c) => (created = c.id)),
    )
    if (ok) {
      adding = false
      await tick()
      canvas?.select(created)
    }
    return ok
  }
</script>

<svelte:head><title>{data ? `${data.node.name} · knotbase` : 'knotbase'}</title></svelte:head>

{#if data}
  {@const node = data.node}
  {@const graph = data.graph}
  <main class="split">
    <section class="canvas-col" aria-label="Graph of {node.name}">
      <!-- shortcut: multi-delete is one request per item, not atomic; it stops at the first failure, add a batch endpoint when partial failure matters. -->
      <NodeFlow
        bind:this={canvas}
        nodes={graph.nodes}
        edges={graph.edges}
        onMove={(ms) => change(Promise.all(ms.map((m) => api.updateNode(m.id, { x: m.x, y: m.y }))))}
        onConnect={(s, t) => {
          if (!graph.edges.some((e) => e.source_id === s && e.target_id === t))
            change(api.createEdge({ parent_id: node.id, source_id: s, target_id: t }))
        }}
        onDelete={(ns, es) =>
          change(
            (async () => {
              for (const n of ns) await api.deleteNode(n)
              for (const e of es) await api.deleteEdge(e)
            })(),
          )}
        onOpen={(cid) => navigate(`/node/${cid}`)}
        onSelect={(n, e) => {
          selectedId = n
          selectedEdgeId = e
        }}
      />
      {#if adding}
        <AddNodeForm levelName={node.name} onAdd={addNode} onCancel={() => (adding = false)} />
      {:else if graph.nodes.length === 0}
        <div class="cover">
          <EmptyCard
            title="Nothing inside {node.name} yet"
            text="Add a part, a flow or a step. Each node gets its own graph, doc and checklist."
          >
            <Button variant="primary" onclick={() => (adding = true)}><Icon name="plus" />Add node</Button>
          </EmptyCard>
        </div>
      {/if}
    </section>
    <aside class="inspector" aria-label="Inspector">
      {#if edge}
        <EdgeInspector
          {edge}
          nodes={graph.nodes}
          levelName={node.name}
          {change}
          onClear={() => {
            selectedEdgeId = null
            canvas?.select(null)
          }}
        />
      {:else}
        {#key selectedId ?? node.id}
          <Inspector
            id={selectedId ?? node.id}
            level={node}
            levelGraph={graph}
            {change}
            onSelect={(i) => canvas?.select(i)}
          />
        {/key}
      {/if}
    </aside>
  </main>
{:else if notify.error}
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
