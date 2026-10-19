<script lang="ts">
  import { tick } from 'svelte'
  import { api, type Graph, type NodeDetail } from '../api'
  import { navigate } from '../nav.svelte'
  import { act, ui } from '../ui.svelte'
  import { plural } from '../flow'
  import TopBar from '../components/TopBar.svelte'
  import ThemeToggle from '../components/ThemeToggle.svelte'
  import Button from '../components/Button.svelte'
  import GraphCanvas from '../graph/GraphCanvas.svelte'
  import AddNodeForm from '../graph/AddNodeForm.svelte'
  import Inspector from '../inspector/Inspector.svelte'
  import EdgeInspector from '../inspector/EdgeInspector.svelte'

  // App keys this page by id, so every level starts with fresh state.
  let { id }: { id: number } = $props()
  let data = $state.raw<{ node: NodeDetail; graph: Graph }>() // raw: never mutated in place, always replaced
  let selectedId = $state<number | null>(null)
  let selectedEdgeId = $state<number | null>(null)
  let adding = $state(false)
  let canvas = $state<ReturnType<typeof GraphCanvas>>()
  const edge = $derived(data?.graph.edges.find(e => e.id === selectedEdgeId))

  let seq = 0 // only the latest reload may write: multi-node drags and inspector edits fire overlapping reloads
  async function reload() {
    const n = ++seq
    await act(Promise.all([api.node(id), api.graph(id)]).then(([node, graph]) => {
      if (n === seq) { data = { node, graph }; document.title = `${node.name} · knotbase` }
    }))
  }
  reload()
  // shortcut: refetch after every mutation, no optimistic state (a failed one still refetches so the UI snaps back, then shows the banner); revisit if the backend is ever remote.
  async function change(p: Promise<unknown>) { const ok = await act(p); await reload(); return ok }

  async function addNode(name: string, kind: string) {
    const n = data?.graph.nodes.length ?? 0
    let created: number | null = null
    // new cards fill a 3-wide grid, as the prototype does
    const ok = await change(api.createNode({ parent_id: id, name, kind, x: (n % 3) * 300, y: 100 + Math.floor(n / 3) * 200 }).then(c => { created = c.id }))
    if (ok) { adding = false; await tick(); canvas?.select(created) }
    return ok
  }
</script>

<TopBar crumbs={data?.node.breadcrumb ?? []}>
  {#if data}<span class="stats">{plural(data.graph.nodes.length, 'node')} · {plural(data.graph.edges.length, 'connection')}</span>{/if}
  <ThemeToggle />
  <Button variant="primary" onclick={() => (adding = !adding)}>
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" aria-hidden="true"><path d="M8 3v10M3 8h10"></path></svg>Add node
  </Button>
</TopBar>
{#if data}
  {@const node = data.node}
  {@const graph = data.graph}
  <div class="split">
    <section class="canvas-col" aria-label="Graph of {node.name}">
      <!-- shortcut: multi-delete is one request per item, not atomic; it stops at the first failure, add a batch endpoint when partial failure matters. -->
      <GraphCanvas bind:this={canvas} nodes={graph.nodes} edges={graph.edges}
        onMove={(ms) => change(Promise.all(ms.map(m => api.updateNode(m.id, { x: m.x, y: m.y }))))}
        onConnect={(s, t) => change(api.createEdge({ parent_id: node.id, source_id: s, target_id: t }))}
        onDelete={(ns, es) => change((async () => { for (const n of ns) await api.deleteNode(n); for (const e of es) await api.deleteEdge(e) })())}
        onOpen={(cid) => navigate(`/node/${cid}`)}
        onSelect={(n, e) => { selectedId = n; selectedEdgeId = e }} />
      {#if adding}
        <AddNodeForm levelName={node.name} onAdd={addNode} onCancel={() => (adding = false)} />
      {:else if graph.nodes.length === 0}
        <div class="cover">
          <div class="empty-card">
            <p><b>Nothing inside {node.name} yet</b></p>
            <p class="muted">Add a part, a flow or a step. Each node gets its own graph, doc and checklist.</p>
            <Button variant="primary" onclick={() => (adding = true)}>Add node</Button>
          </div>
        </div>
      {/if}
    </section>
    <aside class="inspector" aria-label="Inspector">
      {#if edge}
        {#key edge.id}
          <EdgeInspector {edge} nodes={graph.nodes} levelName={node.name} onChange={reload} onClear={() => { selectedEdgeId = null; canvas?.select(null) }} />
        {/key}
      {:else}
        {#key selectedId ?? node.id}
          <Inspector id={selectedId ?? node.id} level={node} levelGraph={graph} onChange={reload} onSelect={(i) => canvas?.select(i)} />
        {/key}
      {/if}
    </aside>
  </div>
{:else if !ui.error}
  <p class="empty">Loading…</p>
{/if}
