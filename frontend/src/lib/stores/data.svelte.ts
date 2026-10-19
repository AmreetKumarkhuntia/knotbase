import { SvelteMap } from 'svelte/reactivity'
import { bus } from '$lib/events/bus'
import type { System } from '$lib/events/types'
import type { Graph, NodeDetail } from '$lib/services/api'

// The one read path for data. A read returns what is cached; a miss schedules the matching get:* event in a microtask
// (never a reactive write during a read), one request per key at a time. Handlers write back through set* and ask for
// refresh() after a mutation; the old value stays on screen until the new one lands.
// shortcut: the cache grows with every node visited; add eviction when memory shows.
const nodes = new SvelteMap<number, NodeDetail>()
const graphs = new SvelteMap<number, Graph>()
let systemList = $state.raw<System[]>()
const status = new SvelteMap<string, 'loading' | { error: string }>()
// eslint-disable-next-line svelte/prefer-svelte-reactivity -- bookkeeping only, nothing renders from it
const inflight = new Map<string, Promise<void>>()

type Key = `node:${number}` | `graph:${number}` | 'systems'

const getFor = (key: Key) => {
  const [kind, id] = key.split(':')
  return kind === 'systems'
    ? bus.emit('get:systems', {})
    : kind === 'graph'
      ? bus.emit('get:graph', { id: Number(id) })
      : bus.emit('get:node', { id: Number(id) })
}
const cached = (key: Key) => {
  const [kind, id] = key.split(':')
  return kind === 'systems'
    ? systemList !== undefined
    : kind === 'graph'
      ? graphs.has(Number(id))
      : nodes.has(Number(id))
}

// runs the get for a key after any request already in flight for it, so a later write never loses to an earlier read
function run(key: Key) {
  const p = (inflight.get(key) ?? Promise.resolve())
    .then(() => {
      status.set(key, 'loading')
      return getFor(key)
    })
    .then(
      () => {
        status.delete(key)
      },
      (e) => {
        status.set(key, { error: (e as Error).message })
      },
    )
    .finally(() => {
      if (inflight.get(key) === p) inflight.delete(key)
    })
  inflight.set(key, p)
  return p
}
function ensure(key: Key, value: unknown) {
  if (value !== undefined || status.has(key) || inflight.has(key)) return
  queueMicrotask(() => {
    if (!inflight.has(key)) run(key)
  })
}

export const data = {
  node(id: number) {
    const v = nodes.get(id)
    ensure(`node:${id}`, v)
    return v
  },
  graph(id: number) {
    const v = graphs.get(id)
    ensure(`graph:${id}`, v)
    return v
  },
  systems() {
    ensure('systems', systemList)
    return systemList
  },
  status: (key: Key) => status.get(key),
  // a read that never schedules a fetch, for handlers that only want what is already here
  peekGraph: (id: number) => graphs.get(id),

  // written by the get handlers only
  setNode: (id: number, v: NodeDetail) => {
    nodes.set(id, v)
  },
  setGraph: (id: number, v: Graph) => {
    graphs.set(id, v)
  },
  setSystems: (s: System[]) => {
    systemList = s
  },
  drop(id: number) {
    nodes.delete(id)
    graphs.delete(id)
    status.delete(`node:${id}`)
    status.delete(`graph:${id}`)
  },
  // after a mutation: re-run the gets for the keys that are on screen (a key nobody has read is fetched on its first read anyway)
  async refresh(...keys: Key[]) {
    await Promise.all(keys.filter(cached).map(run))
  },
}
