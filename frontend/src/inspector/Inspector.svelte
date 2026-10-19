<script lang="ts">
  import { api, type Graph, type NodeDetail } from '../api'
  import { navigate } from '../nav.svelte'
  import { act } from '../ui.svelte'
  import { plural } from '../flow'
  import Button from '../components/Button.svelte'
  import Swatch from '../components/Swatch.svelte'
  import InsideSection from './InsideSection.svelte'
  import DocSection from './DocSection.svelte'
  import ItemsSection from './ItemsSection.svelte'

  // Inspects the level itself (id === level.id) or a selected card; the parent keys this component by id.
  let { id, level, levelGraph, onChange, onSelect }: {
    id: number; level: NodeDetail; levelGraph: Graph; onChange: () => Promise<void>; onSelect: (id: number | null) => void
  } = $props()
  const isLevel = $derived(id === level.id)
  let node = $state.raw<NodeDetail>()
  let graph = $state.raw<Graph>()
  let renaming = $state(false)
  let draft = $state('')
  let confirming = $state(false)

  let seq = 0 // only the latest reload may write
  async function reload() {
    const n = ++seq
    // shortcut: when id === level.id this repeats the page's own fetch; local and cheap, share the data if it ever shows.
    await act(Promise.all([api.node(id), api.graph(id)]).then(([d, g]) => { if (n === seq) { node = d; graph = g } }))
  }
  reload()
  // shortcut: refetch after every mutation, no optimistic state (a failed one still refetches so the UI snaps back, then shows the banner).
  async function change(p: Promise<unknown>) {
    const ok = await act(p)
    await Promise.all([reload(), onChange()])
    return ok
  }
  async function rename(e: SubmitEvent) {
    e.preventDefault()
    if (draft.trim() && (await change(api.updateNode(id, { name: draft.trim() })))) renaming = false
  }
  async function remove() {
    if (!(await act(api.deleteNode(id)))) return
    if (isLevel) { const up = level.breadcrumb.at(-2); navigate(up && up.id !== 1 ? `/node/${up.id}` : '/') }
    else { onSelect(null); await onChange() }
  }
  const meta = $derived.by(() => {
    if (!graph) return ''
    if (isLevel) return `${plural(levelGraph.nodes.length, 'node')} · ${plural(levelGraph.edges.length, 'connection')}`
    const conn = levelGraph.edges.filter(e => e.source_id === id || e.target_id === id).length
    return `${graph.nodes.length} inside · ${plural(conn, 'connection')} at this level`
  })
</script>

{#if node && graph}
  <div class="head">
    <span class="kindline">
      <Swatch kind={node.kind} />{node.kind}<span class="spacer"></span>
      {#if isLevel}<span>this level</span>{:else}<span>selected</span><Button class="small" onclick={() => onSelect(null)}>Clear</Button>{/if}
    </span>
    {#if renaming}
      <form class="row" onsubmit={rename}>
        <label class="sr-only" for="rename">Name</label>
        <input id="rename" class="grow" {@attach (el) => el.focus()} bind:value={draft} required />
        <Button variant="primary" type="submit">Save</Button>
        <Button onclick={() => (renaming = false)}>Cancel</Button>
      </form>
    {:else}
      <h1>{node.name}</h1>
    {/if}
    <p class="meta">{meta}</p>
    {#if confirming}
      <div class="confirm">
        <span>Delete <b>{node.name}</b> and everything inside it? This can't be undone.</span>
        <div class="row">
          <Button variant="danger" class="fill" onclick={remove}>Delete</Button>
          <Button onclick={() => (confirming = false)}>Cancel</Button>
        </div>
      </div>
    {:else}
      <div class="row">
        {#if !isLevel}
          <Button variant="primary" onclick={() => navigate(`/node/${id}`)}>Open {node.name}<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 8h10M9 4l4 4-4 4"></path></svg></Button>
        {/if}
        <Button onclick={() => { draft = node?.name ?? ''; renaming = true }}>Rename</Button>
        <Button variant="danger" onclick={() => { confirming = true; renaming = false }}>Delete</Button>
      </div>
    {/if}
  </div>
  <!-- shortcut: picking a child of a selected card opens the card rather than selecting the child inside it; carry the child id through navigate() when wanted. -->
  <InsideSection nodes={graph.nodes} onPick={(cid) => (isLevel ? onSelect(cid) : navigate(`/node/${id}`))} />
  <DocSection doc={node.doc} onSave={(doc) => change(api.updateNode(id, { doc }))} />
  <ItemsSection items={node.items}
    onAdd={(text) => change(api.createItem({ node_id: id, text }))}
    onToggle={(it) => change(api.updateItem(it.id, { done: !it.done }))}
    onDelete={(iid) => change(api.deleteItem(iid))} />
{/if}
