<script lang="ts">
  import { Handle, Position, type Node, type NodeProps } from '@xyflow/svelte'
  import Link from './Link.svelte'

  type Card = Node<{ name: string; kind: string }, 'card'>
  let { id, data, selected }: NodeProps<Card> = $props()
</script>

<div class="card" class:selected style:--kind="var(--kind-{data.kind}, var(--kind-default))">
  <Handle type="target" position={Position.Left} />
  <span>{data.name}</span>
  <span class="chip">{data.kind}</span>
  <!-- keyboard and cmd-click path in; double-click on the card does the same -->
  <Link href="/node/{id}" class="open nodrag" aria-label="Open {data.name}">open</Link>
  <Handle type="source" position={Position.Right} />
</div>
