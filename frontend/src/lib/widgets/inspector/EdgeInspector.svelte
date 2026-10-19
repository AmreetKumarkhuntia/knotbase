<script lang="ts">
  import { api, type ApiEdge, type ApiNode } from '$lib/services/api'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'

  let {
    edge,
    nodes,
    levelName,
    change,
    onClear,
  }: {
    edge: ApiEdge
    nodes: ApiNode[]
    levelName: string
    change: (p: Promise<unknown>) => Promise<boolean>
    onClear: () => void
  } = $props()
  let label = $derived(edge.label) // writable: the draft resets when another edge is selected
  const name = (id: number) => nodes.find((n) => n.id === id)?.name ?? '?'

  async function save(e: SubmitEvent) {
    e.preventDefault()
    await change(api.updateEdge(edge.id, { label: label.trim() }))
  }
  async function remove() {
    if (await change(api.deleteEdge(edge.id))) onClear()
  }
</script>

<div class="head">
  <span class="kindline">
    <Icon name="arrow" size={14} />connection<span class="spacer"></span><Button size="sm" onclick={onClear}
      >Clear</Button
    >
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

<style>
  h1 .arrow {
    font-weight: 400;
  }
</style>
