import { getContext, setContext } from 'svelte'
import { nodes } from '$lib/services/nodes'
import { act } from './notify.svelte'
import { navigate } from './router.svelte'
import { NodeData } from './node-data.svelte'

export type Selection = { type: 'node' | 'edge'; id: number } | null

// Everything the node page and its widgets share: the level's data, the one selection, the add-node popover, and the mutations.
// shortcut: refetch after every mutation, no optimistic state; a failed one still refetches so the UI snaps back, and the banner stays.
export class LevelState {
  level: NodeData
  selection = $state<Selection>(null)
  adding = $state(false)
  #id: () => number

  constructor(id: () => number) {
    this.#id = id
    this.level = new NodeData(id)
  }
  get id() {
    return this.#id()
  }
  get node() {
    return this.level.data?.node
  }
  get graph() {
    return this.level.data?.graph
  }

  select = (sel: Selection) => {
    if (sel?.type !== this.selection?.type || sel?.id !== this.selection?.id) this.selection = sel
  }

  // run a mutation, then refetch the level and, for a selected card, its own data, in parallel
  change = async (fn: () => Promise<unknown>, also?: NodeData) => {
    const ok = await act(fn())
    await Promise.all([this.level.reload(), also?.reload()])
    return ok
  }
  addNode = async (name: string, kind: string) => {
    let created = 0
    const ok = await this.change(() =>
      nodes.add(this.id, name, kind, this.graph?.nodes.length ?? 0).then((n) => (created = n.id)),
    )
    if (ok) {
      this.adding = false
      this.select({ type: 'node', id: created })
    }
    return ok
  }
  move = (moves: { id: number; x: number; y: number }[]) => this.change(() => nodes.move(moves))
  connect = (source: number, target: number) => {
    const p = this.graph && nodes.connect(this.id, this.graph, source, target)
    if (p) this.change(() => p)
  }
  remove = (nodeIds: number[], edgeIds: number[]) => this.change(() => nodes.removeMany(nodeIds, edgeIds))
  // deleting the level itself goes up a crumb instead of refetching a node that is gone
  removeNode = async (id: number) => {
    if (id === this.id) {
      if (await act(nodes.remove(id))) {
        const up = this.node?.breadcrumb.at(-2)
        navigate(up && up.id !== 1 ? `/node/${up.id}` : '/')
      }
    } else if (await this.change(() => nodes.remove(id))) this.select(null)
  }
  setEdgeLabel = (id: number, label: string) => this.change(() => nodes.setEdgeLabel(id, label))
  removeEdge = async (id: number) => {
    if (await this.change(() => nodes.removeEdge(id))) this.select(null)
  }
}

const KEY = Symbol('level')
export const provideLevel = (level: LevelState) => setContext(KEY, level)
export const useLevel = () => getContext<LevelState>(KEY)
