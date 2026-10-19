<script lang="ts">
  import { untrack } from 'svelte'
  import { useStore } from '@xyflow/svelte'
  import { useLevel } from '$lib/stores/level.svelte'

  // Pushes LevelState.selection into xyflow through its own selection API (writing the bound arrays from an effect is not
  // picked up by its memoised selection). Must be rendered inside <SvelteFlow>. Applies only when the primary selected
  // element differs, so a box multi-select survives; xyflow reports the result back through onselectionchange.
  const store = useStore()
  const level = useLevel()

  $effect(() => {
    const sel = level.selection
    untrack(() => {
      const node = store.selectedNodes[0]?.id
      const edge = store.selectedEdges[0]?.id
      const matches =
        sel == null
          ? node === undefined && edge === undefined
          : sel.type === 'node'
            ? node === String(sel.id)
            : node === undefined && edge === String(sel.id)
      if (matches) return
      if (sel == null) store.unselectNodesAndEdges()
      else if (sel.type === 'node') store.addSelectedNodes([String(sel.id)])
      else store.addSelectedEdges([String(sel.id)])
    })
  })
</script>
