import { bus } from '$lib/events/bus'
import { data } from '$lib/stores/data.svelte'
import { act } from '$lib/stores/notify.svelte'
import { api } from './api'

export function register() {
  bus.on('post:edge', async ({ parent_id, source_id, target_id }) => {
    // xyflow drops a repeated connection locally but still reports it, and the schema has no unique constraint
    if (data.peekGraph(parent_id)?.edges.some((e) => e.source_id === source_id && e.target_id === target_id))
      return false
    const ok = await act(api.createEdge({ parent_id, source_id, target_id }))
    await data.refresh(`graph:${parent_id}`)
    return ok
  })
  bus.on('patch:edge', async ({ id, parent_id, label }) => {
    const ok = await act(api.updateEdge(id, { label }))
    await data.refresh(`graph:${parent_id}`)
    return ok
  })
  bus.on('delete:edge', async ({ id, parent_id }) => {
    const ok = await act(api.deleteEdge(id))
    await data.refresh(`graph:${parent_id}`)
    return ok
  })
}
