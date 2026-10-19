import { bus } from '$lib/events/bus'
import { data } from '$lib/stores/data.svelte'
import { act } from '$lib/stores/notify.svelte'
import { api, type ApiNode } from './api'

// the keys a change under `parent` shows up in: its graph, and the Home grid when the parent is the root
const under = (parent: number | null) =>
  parent == null ? [] : parent === 1 ? ([`graph:${parent}`, 'systems'] as const) : ([`graph:${parent}`] as const)

export function register() {
  bus.on('get:node', async ({ id }) => data.setNode(id, await api.node(id)))
  bus.on('get:graph', async ({ id }) => data.setGraph(id, await api.graph(id)))

  bus.on('post:node', async ({ parent_id, name, kind }) => {
    // new cards fill a 3-wide grid, as the design does
    const n = data.peekGraph(parent_id)?.nodes.length ?? 0
    let created: ApiNode | undefined
    const ok = await act(
      api
        .createNode({ parent_id, name, kind, x: (n % 3) * 300, y: 100 + Math.floor(n / 3) * 200 })
        .then((c) => (created = c)),
    )
    await data.refresh(...under(parent_id))
    return ok ? created : undefined
  })
  bus.on('patch:node', async ({ id, patch }) => {
    let parent: number | null = null
    const ok = await act(api.updateNode(id, patch).then((n) => (parent = n.parent_id)))
    await data.refresh(`node:${id}`, ...under(parent))
    return ok
  })
  bus.on('patch:nodes', async ({ parent_id, moves }) => {
    const ok = await act(Promise.all(moves.map((m) => api.updateNode(m.id, { x: m.x, y: m.y }))))
    await data.refresh(...under(parent_id)) // a failed move snaps the cards back
    return ok
  })
  bus.on('delete:node', async ({ id, parent_id }) => {
    const ok = await act(api.deleteNode(id))
    await data.refresh(...under(parent_id))
    if (ok) data.drop(id) // after the refresh: until the graph no longer lists it, its inspector would refetch a node that is gone
    return ok
  })
  // shortcut: one request per item, not atomic; it stops at the first failure, add a batch endpoint when partial failure matters.
  bus.on('delete:many', async ({ parent_id, nodeIds, edgeIds }) => {
    const ok = await act(
      (async () => {
        for (const n of nodeIds) await api.deleteNode(n)
        for (const e of edgeIds) await api.deleteEdge(e)
      })(),
    )
    await data.refresh(...under(parent_id))
    nodeIds.forEach(data.drop)
    return ok
  })
}
