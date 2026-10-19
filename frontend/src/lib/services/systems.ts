import { bus } from '$lib/events/bus'
import { data } from '$lib/stores/data.svelte'
import { act } from '$lib/stores/notify.svelte'
import { api } from './api'

// Systems are the children of root 1; the root itself is never shown.
export function register() {
  // shortcut: 2N+1 requests (root graph, then node + graph per system); replace with GET /api/home when the backend grows it.
  bus.on('get:systems', async () => {
    const g = await api.graph(1)
    data.setSystems(
      await Promise.all(
        g.nodes.map(async (n) => {
          const [detail, graph] = await Promise.all([api.node(n.id), api.graph(n.id)])
          return { node: n, doc: detail.doc, inside: graph.nodes }
        }),
      ),
    )
  })
  // shortcut: two requests because POST /api/nodes takes no doc yet; a failed doc PATCH rolls the new system back so a retry can't duplicate it.
  bus.on('post:system', async ({ name, desc }) => {
    let id = 0
    const ok = await act(
      api.createNode({ parent_id: 1, name, kind: 'system' }).then(async (n) => {
        id = n.id
        if (desc)
          await api.updateNode(n.id, { doc: desc }).catch(async (e) => {
            await api.deleteNode(n.id).catch(() => {})
            throw e
          })
      }),
    )
    await data.refresh('systems', 'graph:1')
    return ok ? id : undefined
  })
}
