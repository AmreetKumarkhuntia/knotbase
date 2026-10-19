<script lang="ts">
  import { Handle, Position, type Node, type NodeProps } from '@xyflow/svelte'
  import Swatch from '$lib/components/Swatch.svelte'
  import { kindColor } from '$lib/utils/kinds'
  import { CARD } from '$lib/utils/layout'
  import type { ApiNode } from '$lib/services/api'

  type Card = Node<{ node: ApiNode }, 'card'>
  let { data, selected }: NodeProps<Card> = $props()
</script>

<!-- keyboard path: Tab to the node, Enter selects it, the inspector's Open button goes inside -->
<div
  class="card"
  class:selected
  title="{data.node.name} · Drag to move · double-click to open"
  style:--kind={kindColor(data.node.kind)}
  style:width="{CARD.w}px"
  style:height="{CARD.h}px"
>
  <Handle type="target" position={Position.Left} />
  <span class="name">{data.node.name}</span>
  <span class="kind"><Swatch kind={data.node.kind} />{data.node.kind}</span>
  <Handle type="source" position={Position.Right} />
</div>

<style>
  .card {
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 6px;
    padding: 17px 16px;
    background: var(--card);
    border: 1px solid var(--card-border);
    border-radius: var(--radius);
    cursor: grab;
  }
  .card.selected {
    padding: 16px 15px;
    border: 2px solid var(--ink);
    box-shadow: 0 0 0 4px color-mix(in srgb, var(--ink) 12%, transparent);
  }
  .name {
    overflow: hidden;
    font-size: 15px;
    font-weight: 600;
    line-height: 20px;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  .kind {
    display: flex;
    align-items: center;
    gap: 6px;
    font: 11px/16px var(--mono);
    color: var(--muted);
    white-space: nowrap;
  }
  :global([data-theme='console']) .card .kind {
    color: var(--kind);
  } /* the console look colours the kind word itself */
</style>
