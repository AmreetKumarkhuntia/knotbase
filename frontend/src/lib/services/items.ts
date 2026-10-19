import { bus } from '$lib/events/bus'
import { data } from '$lib/stores/data.svelte'
import { act } from '$lib/stores/notify.svelte'
import { api } from './api'

// items live inside a node's detail, so every item change refreshes that node
export function register() {
  bus.on('post:item', async ({ node_id, text }) => {
    const ok = await act(api.createItem({ node_id, text }))
    await data.refresh(`node:${node_id}`)
    return ok
  })
  bus.on('patch:item', async ({ id, node_id, done }) => {
    const ok = await act(api.updateItem(id, { done }))
    await data.refresh(`node:${node_id}`)
    return ok
  })
  bus.on('delete:item', async ({ id, node_id }) => {
    const ok = await act(api.deleteItem(id))
    await data.refresh(`node:${node_id}`)
    return ok
  })
}
