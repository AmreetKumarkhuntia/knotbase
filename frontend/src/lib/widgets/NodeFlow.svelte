<script lang="ts">
  import { untrack } from 'svelte'
  import {
    SvelteFlow,
    Background,
    Controls,
    ControlButton,
    MarkerType,
    type Node,
    type Edge,
    type NodeTypes,
    type Connection,
  } from '@xyflow/svelte'
  import CardNode from '$lib/components/CardNode.svelte'
  import Legend from '$lib/components/Legend.svelte'
  import HelpPanel from '$lib/components/HelpPanel.svelte'
  import AddNodeForm from '$lib/components/AddNodeForm.svelte'
  import Icon from '$lib/components/Icon.svelte'
  import SelectionSync from './SelectionSync.svelte'
  import { bus } from '$lib/events/bus'
  import { useLevel } from '$lib/stores/level.svelte'
  import '$lib/styles/xyflow.css'

  // The canvas for the current level: xyflow fed from the level's graph, every change emitted as an event.
  const level = useLevel()
  const nodeTypes: NodeTypes = { card: CardNode }
  let rfNodes = $state.raw<Node[]>([])
  let rfEdges = $state.raw<Edge[]>([])
  let showHelp = $state(false)

  $effect(() => {
    // server truth wins after every refresh; the previous element is carried over so selection and measured size survive
    // shortcut: a refresh landing mid-drag resets that drag; fine single-user with no polling, skip the overwrite while dragging if polling arrives.
    const graph = level.graph
    if (!graph) return
    const prevN = new Map(untrack(() => rfNodes).map((n) => [n.id, n]))
    const prevE = new Map(untrack(() => rfEdges).map((e) => [e.id, e]))
    const name = new Map(graph.nodes.map((n) => [n.id, n.name]))
    rfNodes = graph.nodes.map((n) => ({
      ...prevN.get(String(n.id)),
      id: String(n.id),
      type: 'card',
      position: { x: n.x, y: n.y },
      data: { node: n },
    }))
    rfEdges = graph.edges.map((e) => ({
      ...prevE.get(String(e.id)),
      id: String(e.id),
      source: String(e.source_id),
      target: String(e.target_id),
      label: e.label || undefined,
      ariaLabel: `Connection from ${name.get(e.source_id)} to ${name.get(e.target_id)}${e.label ? ': ' + e.label : ''}`,
      type: 'smoothstep',
      markerEnd: { type: MarkerType.ArrowClosed, color: 'var(--edge)' },
    }))
  })

  const move = (nodes: Node[]) =>
    bus.emit('patch:nodes', {
      parent_id: level.id,
      moves: nodes.map((n) => ({ id: Number(n.id), x: n.position.x, y: n.position.y })),
    })
  async function add(name: string, kind: string) {
    const created = await bus.emit('post:node', { parent_id: level.id, name, kind })
    if (created) {
      level.adding = false
      level.select({ type: 'node', id: created.id })
    }
    return !!created
  }
</script>

<!-- arrow keys move selected nodes without firing onnodedragstop, so persist them on keyup -->
<div
  class="canvas"
  role="presentation"
  onkeyup={(e) => {
    const moved = e.key.startsWith('Arrow') ? rfNodes.filter((n) => n.selected) : []
    if (moved.length) move(moved)
  }}
>
  <SvelteFlow
    bind:nodes={rfNodes}
    bind:edges={rfEdges}
    {nodeTypes}
    fitView
    fitViewOptions={{ maxZoom: 1 }}
    isValidConnection={(c) => c.source !== c.target}
    onnodedragstop={({ nodes }) => move(nodes)}
    onconnect={(c: Connection) =>
      bus.emit('post:edge', { parent_id: level.id, source_id: Number(c.source), target_id: Number(c.target) })}
    onbeforedelete={async ({ nodes, edges }) =>
      confirm(
        nodes.length
          ? `Delete ${nodes.length} node(s) and everything inside them?`
          : `Remove ${edges.length} connection(s)?`,
      )}
    ondelete={({ nodes, edges }) => {
      const gone = new Set(nodes.map((n) => n.id)) // cascaded edges must not be deleted twice
      bus.emit('delete:many', {
        parent_id: level.id,
        nodeIds: nodes.map((n) => Number(n.id)),
        edgeIds: edges.filter((e) => !gone.has(e.source) && !gone.has(e.target)).map((e) => Number(e.id)),
      })
    }}
    onnodeclick={({ node, event }) => {
      if (event.detail === 2) bus.emit('nav:node', { id: Number(node.id) })
    }}
    onselectionchange={({ nodes, edges }) =>
      level.select(
        nodes[0]
          ? { type: 'node', id: Number(nodes[0].id) }
          : edges[0]
            ? { type: 'edge', id: Number(edges[0].id) }
            : null,
      )}
  >
    <!-- a box-select also picks the connected edges; the card wins -->
    <Background />
    <SelectionSync />
    <Controls position="bottom-right" showLock={false} aria-label="Canvas controls">
      {#snippet after()}
        <ControlButton
          onclick={() => (showHelp = !showHelp)}
          aria-label="How to use"
          aria-pressed={showHelp}
          title="How to use"
        >
          <Icon name="help" class="help-icon" />
        </ControlButton>
      {/snippet}
    </Controls>
  </SvelteFlow>
  <Legend />
  {#if showHelp}<HelpPanel />{/if}
  {#if level.adding}
    <AddNodeForm levelName={level.node?.name ?? ''} onAdd={add} onCancel={() => (level.adding = false)} />
  {/if}
</div>

<style>
  .canvas {
    position: relative;
    flex: 1 1 auto;
    min-height: 600px;
  }
</style>
