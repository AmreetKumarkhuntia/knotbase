export type Node = { id: number; parent_id: number | null; name: string; kind: string; doc: string; x: number; y: number; created_at: string; updated_at: string }
export type Edge = { id: number; parent_id: number; source_id: number; target_id: number; label: string; created_at: string; updated_at: string }
export type Item = { id: number; node_id: number; text: string; done: boolean; created_at: string; updated_at: string }
export type Graph = { nodes: Node[]; edges: Edge[] }
export type NodeDetail = Node & { items: Item[]; breadcrumb: { id: number; name: string }[] }

async function req<T>(url: string, method = 'GET', body?: unknown): Promise<T> {
  const res = await fetch(url, { method, headers: { 'content-type': 'application/json' }, body: JSON.stringify(body) }) // undefined body stays undefined
  if (!res.ok) {
    // backend answers {"error": msg}; Axum rejections are plain text; the Vite proxy answers an empty 502 when Rust is down
    const text = await res.text()
    let msg = text
    try { msg = JSON.parse(text).error } catch { /* not JSON, keep the raw text */ }
    throw new Error(msg || `${method} ${url} → ${res.status}`)
  }
  return res.status === 204 ? (undefined as T) : res.json()
}

export const api = {
  graph: (id: number) => req<Graph>(`/api/nodes/${id}/graph`),
  node: (id: number) => req<NodeDetail>(`/api/nodes/${id}`),
  createNode: (b: { parent_id: number; name: string; kind: string; x?: number; y?: number }) => req<Node>('/api/nodes', 'POST', b),
  updateNode: (id: number, b: Partial<Pick<Node, 'name' | 'kind' | 'doc' | 'x' | 'y'>>) => req<Node>(`/api/nodes/${id}`, 'PATCH', b),
  deleteNode: (id: number) => req<void>(`/api/nodes/${id}`, 'DELETE'),
  createEdge: (b: { parent_id: number; source_id: number; target_id: number; label?: string }) => req<Edge>('/api/edges', 'POST', b),
  updateEdge: (id: number, b: { label?: string }) => req<Edge>(`/api/edges/${id}`, 'PATCH', b),
  deleteEdge: (id: number) => req<void>(`/api/edges/${id}`, 'DELETE'),
  createItem: (b: { node_id: number; text: string }) => req<Item>('/api/items', 'POST', b),
  updateItem: (id: number, b: { text?: string; done?: boolean }) => req<Item>(`/api/items/${id}`, 'PATCH', b),
  deleteItem: (id: number) => req<void>(`/api/items/${id}`, 'DELETE'),
}
