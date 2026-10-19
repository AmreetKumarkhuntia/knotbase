<script lang="ts">
  import { api, type Node } from '../api'
  import { navigate } from '../nav.svelte'
  import { act, ui } from '../ui.svelte'
  import { plural } from '../flow'
  import TopBar from '../components/TopBar.svelte'
  import ThemeToggle from '../components/ThemeToggle.svelte'
  import Button from '../components/Button.svelte'
  import SystemCard from '../home/SystemCard.svelte'
  import NewSystemForm from '../home/NewSystemForm.svelte'

  type System = { node: Node; doc: string; children: Node[] }
  let systems = $state.raw<System[]>()
  let adding = $state(false)

  // Systems are the children of root 1; the root itself is never shown.
  // shortcut: 2N+1 requests (root graph, then node + graph per system); replace with GET /api/home when the backend grows it.
  act(api.graph(1).then(g => Promise.all(g.nodes.map(async n => {
    const [detail, graph] = await Promise.all([api.node(n.id), api.graph(n.id)])
    return { node: n, doc: detail.doc, children: graph.nodes }
  }))).then(s => { systems = s }))
  document.title = 'knotbase'

  async function create(name: string, desc: string) {
    let id = 0
    // shortcut: two requests because POST /api/nodes takes no doc yet; send it in the POST once the backend accepts it.
    const ok = await act(api.createNode({ parent_id: 1, name, kind: 'system' }).then(async n => { id = n.id; if (desc) await api.updateNode(n.id, { doc: desc }) }))
    if (ok) navigate(`/node/${id}`)
    return ok
  }
</script>

<TopBar>
  {#if systems}<span class="stats">{plural(systems.length, 'system')}</span>{/if}
  <ThemeToggle />
  <Button variant="primary" onclick={() => (adding = !adding)}>
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" aria-hidden="true"><path d="M8 3v10M3 8h10"></path></svg>New system
  </Button>
</TopBar>
<main class="home">
  <section class="systems" aria-labelledby="systems-h">
    <div class="stack tight">
      <h1 id="systems-h">Systems</h1>
      <p class="muted">Independent maps. Each one has its own graph, docs and items.</p>
    </div>
    {#if adding}<NewSystemForm onCreate={create} onCancel={() => (adding = false)} />{/if}
    {#if systems}
      <div class="grid">
        {#each systems as s (s.node.id)}<SystemCard node={s.node} doc={s.doc} children={s.children} />{/each}
        <button class="new-card" onclick={() => (adding = true)}>
          <svg width="24" height="24" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"><path d="M8 3v10M3 8h10"></path></svg>
          New system
        </button>
      </div>
    {:else if !ui.error}
      <p class="empty">Loading…</p>
    {/if}
  </section>
</main>
