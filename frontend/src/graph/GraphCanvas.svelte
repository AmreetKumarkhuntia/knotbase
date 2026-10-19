<script lang="ts">
  import { untrack } from 'svelte'
  import { SvelteFlow, Background, Controls, ControlButton, MarkerType, type Node, type Edge as FlowEdge, type NodeTypes, type Connection } from '@xyflow/svelte'
  import CardNode from './CardNode.svelte'
  import Legend from './Legend.svelte'
  import HelpPanel from './HelpPanel.svelte'
  import { theme } from '../theme.svelte'
  import type { Node as ApiNode, Edge as ApiEdge } from '../api'

  let { nodes, edges, onMove, onConnect, onDelete, onOpen, onSelect }: {
    nodes: ApiNode[]; edges: ApiEdge[]
    onMove: (moves: { id: number; x: number; y: number }[]) => void; onConnect: (sourceId: number, targetId: number) => void
    onDelete: (nodeIds: number[], edgeIds: number[]) => void; onOpen: (id: number) => void
    onSelect: (nodeId: number | null, edgeId: number | null) => void
  } = $props()

  const nodeTypes: NodeTypes = { card: CardNode }
  const EDGE = { notebook: '#55645c', console: '#6f7d8b' } // the arrow marker is an inline attribute, so it can't read a CSS var
  let rfNodes = $state.raw<Node[]>([])
  let rfEdges = $state.raw<FlowEdge[]>([])
  let showHelp = $state(false)

  $effect(() => { // server truth wins after every refetch
    // carry the previous element over so selection and measured size survive; the server position stays authoritative
    // shortcut: a refetch landing mid-drag resets that drag; fine single-user with no polling, skip the overwrite while dragging if polling arrives.
    const prevN = new Map(untrack(() => rfNodes).map(n => [n.id, n]))
    const prevE = new Map(untrack(() => rfEdges).map(e => [e.id, e]))
    rfNodes = nodes.map(n => ({ ...prevN.get(String(n.id)), id: String(n.id), type: 'card', position: { x: n.x, y: n.y }, data: { node: n } }))
    rfEdges = edges.map(e => ({ ...prevE.get(String(e.id)), id: String(e.id), source: String(e.source_id), target: String(e.target_id), label: e.label || undefined,
                                type: 'smoothstep', markerEnd: { type: MarkerType.ArrowClosed, color: EDGE[theme.name] } }))
  })

  // programmatic selection (Inside list, a just-added node); xyflow reports it back through onselectionchange
  export function select(id: number | null) {
    const sid = id == null ? null : String(id)
    rfNodes = rfNodes.map(n => ({ ...n, selected: n.id === sid }))
    rfEdges = rfEdges.map(e => ({ ...e, selected: false }))
  }
</script>

<!-- arrow keys move selected nodes without firing onnodedragstop, so persist them on keyup -->
<div class="canvas" role="presentation" onkeyup={(e) => {
  const moved = e.key.startsWith('Arrow') ? rfNodes.filter(n => n.selected) : []
  if (moved.length) onMove(moved.map(n => ({ id: Number(n.id), x: n.position.x, y: n.position.y })))
}}>
  <SvelteFlow bind:nodes={rfNodes} bind:edges={rfEdges} {nodeTypes} fitView fitViewOptions={{ maxZoom: 1 }}
    colorMode={theme.name === 'console' ? 'dark' : 'light'}
    isValidConnection={(c) => c.source !== c.target}
    onnodedragstop={({ nodes }) => onMove(nodes.map(n => ({ id: Number(n.id), x: n.position.x, y: n.position.y })))}
    onconnect={(c: Connection) => onConnect(Number(c.source), Number(c.target))}
    onbeforedelete={async ({ nodes }) => nodes.length === 0 || confirm(`Delete ${nodes.length} node(s) and everything inside them?`)}
    ondelete={({ nodes, edges }) => {
      const gone = new Set(nodes.map(n => n.id)) // cascaded edges must not be deleted twice
      onDelete(nodes.map(n => Number(n.id)), edges.filter(e => !gone.has(e.source) && !gone.has(e.target)).map(e => Number(e.id)))
    }}
    onnodeclick={({ node, event }) => { if (event.detail === 2) onOpen(Number(node.id)) }}
    onselectionchange={({ nodes, edges }) => onSelect(nodes[0] ? Number(nodes[0].id) : null, edges[0] ? Number(edges[0].id) : null)}>
    <Background />
    <Controls position="bottom-right" showLock={false} aria-label="Canvas controls">
      {#snippet after()}
        <ControlButton onclick={() => (showHelp = !showHelp)} aria-label="How to use" aria-pressed={showHelp} title="How to use">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="8" cy="8" r="6.25"></circle><path d="M6.2 6.3a1.9 1.9 0 0 1 3.7.4c0 1.2-1.9 1.4-1.9 2.6M8 11.6h.01"></path></svg>
        </ControlButton>
      {/snippet}
    </Controls>
  </SvelteFlow>
  <Legend />
  {#if showHelp}<HelpPanel />{/if}
</div>
