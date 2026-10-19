<script lang="ts">
  import { api, type Graph, type NodeDetail } from './api'
  import { navigate } from './nav.svelte'
  import TopBar from './TopBar.svelte'
  import GraphCanvas from './GraphCanvas.svelte'
  import DocPanel from './DocPanel.svelte'
  import ItemList from './ItemList.svelte'

  let { id }: { id: number } = $props()
  let data = $state.raw<{ node: NodeDetail; graph: Graph }>() // raw: never mutated in place, always replaced
  let error = $state('')
  let name = $state('')
  let kind = $state('')

  let seq = 0 // only the latest reload may write: fast crumb clicks and multi-node drags fire overlapping reloads
  async function reload() {
    const n = ++seq, my = id
    try {
      const [node, graph] = await Promise.all([api.node(my), api.graph(my)])
      if (n === seq) { data = { node, graph }; error = ''; document.title = `${node.name} · knotbase` }
    } catch (e) {
      if (n !== seq) return
      error = (e as Error).message
      if (data?.node.id !== my) data = undefined // never show another node's page under this URL; same node keeps data so an unsaved doc draft survives
    }
  }
  // shortcut: refetch after every mutation, no optimistic state (a failed one still refetches so the UI snaps back, then shows the banner); revisit if the backend is ever remote.
  async function act(p: Promise<unknown>) {
    try { await p; await reload(); return true }
    catch (e) { const msg = (e as Error).message; await reload(); error = msg; return false }
  }
  $effect(() => { id; reload() }) // re-runs when the route id changes
  // shortcut: new nodes stagger from top-left; use screenToFlowPosition via useSvelteFlow() when that annoys.
  async function addNode(e: SubmitEvent) {
    e.preventDefault()
    if (!data) return // `id` can already be the next node while `data` is still the old one, so write against `data`
    const n = data.graph.nodes.length
    if (await act(api.createNode({ parent_id: data.node.id, name: name.trim(), kind: kind.trim(), x: 40 + 32 * n, y: 40 + 32 * n }))) name = ''
  }
</script>

<TopBar crumbs={data?.node.breadcrumb ?? []} />
{#if error}<p class="error" role="alert">{error}</p>{/if}
{#if data}
  {@const node = data.node}
  {@const graph = data.graph}
  <main>
    <header class="head">
      <h1>{node.name}</h1>
      <span class="chip" style:--kind="var(--kind-{node.kind}, var(--kind-default))">{node.kind}</span>
    </header>
    <form class="row" onsubmit={addNode}>
      <label class="field">Name <input bind:value={name} required /></label>
      <label class="field">Kind <input bind:value={kind} list="kinds" required /></label>
      <datalist id="kinds">
        {#each ['frontend', 'backend', 'database', 'external', 'service', 'flow', 'step'] as k}<option value={k}></option>{/each}
      </datalist>
      <button class="primary">Add node</button>
    </form>
    <p class="hint">Double-click a node to go inside. Drag from one handle to another to connect. Select and press Backspace to delete.</p>
    <!-- shortcut: connecting nodes needs a mouse drag between handles; add a "Connect selected" action when keyboard-only editing is needed -->
    <a class="skip" href="#doc-h">Skip graph</a>
    <!-- keyed by the loaded node so each level gets a fresh viewport (fitView) and fresh editor state -->
    {#key node.id}
      <div class="graph">
        <!-- shortcut: multi-delete is one request per item, not atomic; it stops at the first failure, add a batch endpoint when partial failure matters. -->
        <GraphCanvas nodes={graph.nodes} edges={graph.edges}
          onMove={(ms) => act(Promise.all(ms.map(m => api.updateNode(m.id, { x: m.x, y: m.y }))))}
          onConnect={(s, t) => act(api.createEdge({ parent_id: node.id, source_id: s, target_id: t }))}
          onDelete={(ns, es) => act((async () => { for (const n of ns) await api.deleteNode(n); for (const e of es) await api.deleteEdge(e) })())}
          onOpen={(cid) => navigate(`/node/${cid}`)} />
        {#if graph.nodes.length === 0}
          <p class="empty cover">No nodes inside yet. Add a part, a flow or a step.</p>
        {/if}
      </div>
      <div class="cols">
        <DocPanel doc={node.doc} onSave={(doc) => act(api.updateNode(node.id, { doc }))} />
        <ItemList items={node.items}
          onAdd={(text) => act(api.createItem({ node_id: node.id, text }))}
          onToggle={(it) => act(api.updateItem(it.id, { done: !it.done }))}
          onDelete={(iid) => act(api.deleteItem(iid))} />
      </div>
    {/key}
  </main>
{:else if !error}
  <p class="empty">Loading…</p>
{/if}
