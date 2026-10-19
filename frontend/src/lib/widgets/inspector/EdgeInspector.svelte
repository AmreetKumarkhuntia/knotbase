<script lang="ts">
  import type { ApiEdge } from '$lib/services/api'
  import { useLevel } from '$lib/stores/level.svelte'
  import Head from '$lib/components/Head.svelte'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'

  let { edge }: { edge: ApiEdge } = $props()
  const level = useLevel()
  let label = $derived(edge.label) // writable: the draft resets when another edge is selected
  const name = (id: number) => level.graph?.nodes.find((n) => n.id === id)?.name ?? '?'
</script>

<Head onClear={() => level.select(null)}>
  {#snippet tag()}<Icon name="arrow" size={14} />connection{/snippet}
  <h1>{name(edge.source_id)} <span class="muted arrow" aria-hidden="true">→</span> {name(edge.target_id)}</h1>
  <p class="meta">Connection inside {level.node?.name}</p>
  <form
    class="field"
    onsubmit={(e) => {
      e.preventDefault()
      level.setEdgeLabel(edge.id, label.trim())
    }}
  >
    <label for="edge-label">Label</label>
    <div class="row">
      <input id="edge-label" class="grow mono" bind:value={label} placeholder="e.g. HTTPS/JSON" />
      <Button variant="primary" type="submit">Save</Button>
    </div>
  </form>
  <div class="row"><Button variant="danger" onclick={() => level.removeEdge(edge.id)}>Remove connection</Button></div>
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
