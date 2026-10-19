import { api, type ApiNode } from '$lib/services/api'

export type System = { node: ApiNode; doc: string; inside: ApiNode[] }

// Systems are the children of root 1; the root itself is never shown.
// shortcut: 2N+1 requests (root graph, then node + graph per system); replace with GET /api/home when the backend grows it.
export async function loadSystems(): Promise<System[]> {
  const g = await api.graph(1)
  return Promise.all(
    g.nodes.map(async (n) => {
      const [detail, graph] = await Promise.all([api.node(n.id), api.graph(n.id)])
      return { node: n, doc: detail.doc, inside: graph.nodes }
    }),
  )
}

// shortcut: two requests because POST /api/nodes takes no doc yet; a failed doc PATCH rolls the new system back so a retry can't duplicate it.
export async function createSystem(name: string, desc: string) {
  const n = await api.createNode({ parent_id: 1, name, kind: 'system' })
  if (desc)
    await api.updateNode(n.id, { doc: desc }).catch(async (e) => {
      await api.deleteNode(n.id).catch(() => {})
      throw e
    })
  return n.id
}
