<script lang="ts">
  import { untrack } from 'svelte'
  import { SvelteFlow, Background, Controls, MarkerType, type Node, type Edge as FlowEdge, type NodeTypes, type Connection } from '@xyflow/svelte'
  import CardNode from './CardNode.svelte'
  import type { Node as ApiNode, Edge as ApiEdge } from './api'

  let { nodes, edges, onMove, onConnect, onDelete, onOpen }: {
    nodes: ApiNode[]; edges: ApiEdge[]
    onMove: (moves: { id: number; x: number; y: number }[]) => void; onConnect: (sourceId: number, targetId: number) => void
    onDelete: (nodeIds: number[], edgeIds: number[]) => void; onOpen: (id: number) => void
  } = $props()

  const nodeTypes: NodeTypes = { card: CardNode }
  let rfNodes = $state.raw<Node[]>([])
  let rfEdges = $state.raw<FlowEdge[]>([])

  $effect(() => { // server truth wins after every refetch
    // carry the previous node over so selection and measured size survive; the server position stays authoritative
    // shortcut: a refetch landing mid-drag resets that drag; fine single-user with no polling, skip the overwrite while dragging if polling arrives.
    const prev = new Map(untrack(() => rfNodes).map(n => [n.id, n]))
    rfNodes = nodes.map(n => ({ ...prev.get(String(n.id)), id: String(n.id), type: 'card', position: { x: n.x, y: n.y }, data: { name: n.name, kind: n.kind } }))
    rfEdges = edges.map(e => ({ id: String(e.id), source: String(e.source_id), target: String(e.target_id), label: e.label || undefined,
                                type: 'smoothstep', markerEnd: { type: MarkerType.ArrowClosed, color: '#22302a' } })) // marker colour is an inline style, so it can't take a CSS var
  })
</script>

<!-- arrow keys move selected nodes without firing onnodedragstop, so persist them on keyup -->
<div class="canvas" role="presentation" onkeyup={(e) => {
  const moved = e.key.startsWith('Arrow') ? rfNodes.filter(n => n.selected) : []
  if (moved.length) onMove(moved.map(n => ({ id: Number(n.id), x: n.position.x, y: n.position.y })))
}}>
  <SvelteFlow bind:nodes={rfNodes} bind:edges={rfEdges} {nodeTypes} fitView fitViewOptions={{ maxZoom: 1 }}
    isValidConnection={(c) => c.source !== c.target}
    onnodedragstop={({ nodes }) => onMove(nodes.map(n => ({ id: Number(n.id), x: n.position.x, y: n.position.y })))}
    onconnect={(c: Connection) => onConnect(Number(c.source), Number(c.target))}
    onbeforedelete={async ({ nodes }) => nodes.length === 0 || confirm(`Delete ${nodes.length} node(s) and everything inside them?`)}
    ondelete={({ nodes, edges }) => {
      const gone = new Set(nodes.map(n => n.id)) // cascaded edges must not be deleted twice
      onDelete(nodes.map(n => Number(n.id)), edges.filter(e => !gone.has(e.source) && !gone.has(e.target)).map(e => Number(e.id)))
    }}
    onnodeclick={({ node, event }) => { if (event.detail === 2) onOpen(Number(node.id)) }}>
    <Background /><Controls />
  </SvelteFlow>
</div>
