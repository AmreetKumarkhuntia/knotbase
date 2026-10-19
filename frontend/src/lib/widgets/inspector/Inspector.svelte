<script lang="ts">
  import { useLevel } from '$lib/stores/level.svelte'
  import NodeInspector from './NodeInspector.svelte'
  import EdgeInspector from './EdgeInspector.svelte'

  // The right-hand panel: the selected connection, else the selected card, else the level itself.
  const level = useLevel()
  const edge = $derived.by(() => {
    const s = level.selection
    return s?.type === 'edge' ? level.graph?.edges.find((e) => e.id === s.id) : undefined
  })
  const nodeId = $derived(level.selection?.type === 'node' ? level.selection.id : level.id)
</script>

{#if edge}
  <EdgeInspector {edge} />
{:else}
  <NodeInspector id={nodeId} />
{/if}
