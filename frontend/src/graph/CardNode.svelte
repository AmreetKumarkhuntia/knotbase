<script lang="ts">
  import { Handle, Position, type Node, type NodeProps } from '@xyflow/svelte'
  import Swatch from '../components/Swatch.svelte'
  import type { Node as ApiNode } from '../api'

  type Card = Node<{ node: ApiNode }, 'card'>
  let { data, selected }: NodeProps<Card> = $props()
</script>

<!-- keyboard path: Tab to the node, Enter selects it, the inspector's Open button goes inside -->
<div class="card" class:selected title="Drag to move · double-click to open" style:--kind="var(--kind-{data.node.kind}, var(--muted))">
  <Handle type="target" position={Position.Left} />
  <span class="name">{data.node.name}</span>
  <span class="kind"><Swatch kind={data.node.kind} />{data.node.kind}</span>
  <Handle type="source" position={Position.Right} />
</div>
