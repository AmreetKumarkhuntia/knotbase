<script lang="ts">
  import type { ApiEdge } from '$lib/services/api'
  import { bus } from '$lib/events/bus'
  import { useLevel } from '$lib/stores/level.svelte'
  import Head from '$lib/components/Head.svelte'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'

  let { edge }: { edge: ApiEdge } = $props()
  const level = useLevel()
  let label = $derived(edge.label) // writable: the draft resets when another edge is selected
  const name = (id: number) => level.graph?.nodes.find((n) => n.id === id)?.name ?? '?'

  async function remove() {
    if (await bus.emit('delete:edge', { id: edge.id, parent_id: edge.parent_id })) level.select(null)
  }
</script>

<Head onClear={() => level.select(null)}>
  {#snippet tag()}<Icon name="arrow" size={14} />connection{/snippet}
  <h1>{name(edge.source_id)} <span class="muted arrow" aria-hidden="true">→</span> {name(edge.target_id)}</h1>
  <p class="meta">Connection inside {level.node?.name}</p>
  <form
    class="field"
    onsubmit={(e) => {
      e.preventDefault()
      bus.emit('patch:edge', { id: edge.id, parent_id: edge.parent_id, label: label.trim() })
    }}
  >
    <label for="edge-label">Label</label>
    <div class="row">
      <input id="edge-label" class="grow mono" bind:value={label} placeholder="e.g. HTTPS/JSON" />
      <Button variant="primary" type="submit">Save</Button>
    </div>
  </form>
  <div class="row"><Button variant="danger" onclick={remove}>Remove connection</Button></div>
</Head>
<p class="note muted">Connections carry a label only. Docs and items belong to the nodes at either end.</p>

<style>
  .meta {
    font-size: 13px;
    color: var(--muted);
  }
  .arrow {
    font-weight: 400;
  }
  .note {
    padding: 16px 20px;
  }
</style>
