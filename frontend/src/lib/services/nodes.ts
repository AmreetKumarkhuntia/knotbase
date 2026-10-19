import { api, type Graph, type Item } from './api'

// Node-level operations; widgets run them through LevelState.change(), which refetches afterwards.
export const nodes = {
  // new cards fill a 3-wide grid, as the design does
  add: (parentId: number, name: string, kind: string, count: number) =>
    api.createNode({ parent_id: parentId, name, kind, x: (count % 3) * 300, y: 100 + Math.floor(count / 3) * 200 }),
  move: (moves: { id: number; x: number; y: number }[]) =>
    Promise.all(moves.map((m) => api.updateNode(m.id, { x: m.x, y: m.y }))),
  // xyflow drops a repeated connection locally but still reports it, and the schema has no unique constraint
  connect: (parentId: number, graph: Graph, source: number, target: number) =>
    graph.edges.some((e) => e.source_id === source && e.target_id === target)
      ? null
      : api.createEdge({ parent_id: parentId, source_id: source, target_id: target }),
  // shortcut: one request per item, not atomic; it stops at the first failure, add a batch endpoint when partial failure matters.
  removeMany: async (nodeIds: number[], edgeIds: number[]) => {
    for (const n of nodeIds) await api.deleteNode(n)
    for (const e of edgeIds) await api.deleteEdge(e)
  },
  remove: (id: number) => api.deleteNode(id),
  rename: (id: number, name: string) => api.updateNode(id, { name }),
  setDoc: (id: number, doc: string) => api.updateNode(id, { doc }),
  addItem: (nodeId: number, text: string) => api.createItem({ node_id: nodeId, text }),
  toggleItem: (item: Item) => api.updateItem(item.id, { done: !item.done }),
  removeItem: (id: number) => api.deleteItem(id),
  setEdgeLabel: (id: number, label: string) => api.updateEdge(id, { label }),
  removeEdge: (id: number) => api.deleteEdge(id),
}
