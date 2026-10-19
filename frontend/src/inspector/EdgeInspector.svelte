<script lang="ts">
  import { api, type Edge, type Node } from '../api'
  import Button from '../components/Button.svelte'

  // the parent keys this component by edge id, so the draft is seeded once
  let { edge, nodes, levelName, change, onClear }: {
    edge: Edge; nodes: Node[]; levelName: string; change: (p: Promise<unknown>) => Promise<boolean>; onClear: () => void
  } = $props()
  // svelte-ignore state_referenced_locally
  let label = $state(edge.label)
  const name = (id: number) => nodes.find(n => n.id === id)?.name ?? '?'

  async function save(e: SubmitEvent) { e.preventDefault(); await change(api.updateEdge(edge.id, { label: label.trim() })) }
  async function remove() { if (await change(api.deleteEdge(edge.id))) onClear() }
</script>

<div class="head">
  <span class="kindline">
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 8h10M9 4l4 4-4 4"></path></svg>
    connection<span class="spacer"></span><Button class="small" onclick={onClear}>Clear</Button>
  </span>
  <h1>{name(edge.source_id)} <span class="muted arrow" aria-hidden="true">→</span> {name(edge.target_id)}</h1>
  <p class="meta">Connection inside {levelName}</p>
  <form class="field" onsubmit={save}>
    <label for="edge-label">Label</label>
    <div class="row">
      <input id="edge-label" class="grow mono" bind:value={label} placeholder="e.g. HTTPS/JSON" />
      <Button variant="primary" type="submit">Save</Button>
    </div>
  </form>
  <div class="row"><Button variant="danger" onclick={remove}>Remove connection</Button></div>
</div>
<p class="section muted">Connections carry a label only. Docs and items belong to the nodes at either end.</p>
