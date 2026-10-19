import { untrack } from 'svelte'
import { api, type Graph, type NodeDetail } from '$lib/services/api'

// A node's detail plus its child graph, reloaded whenever id() changes or reload() is called.
// Construct it during component init: it owns an $effect.
export class NodeData {
  data = $state.raw<{ node: NodeDetail; graph: Graph }>()
  error = $state('')
  #id: () => number | null
  #seq = 0 // only the latest reload may write

  constructor(id: () => number | null) {
    this.#id = id
    $effect(() => {
      this.#id()
      untrack(() => this.reload())
    })
  }

  async reload() {
    const n = ++this.#seq
    const id = this.#id()
    if (id == null) {
      this.data = undefined
      return
    }
    try {
      const [node, graph] = await Promise.all([api.node(id), api.graph(id)])
      if (n === this.#seq) {
        this.data = { node, graph }
        this.error = ''
      }
    } catch (e) {
      if (n === this.#seq) {
        this.data = undefined
        this.error = (e as Error).message
      }
    }
  }
}
