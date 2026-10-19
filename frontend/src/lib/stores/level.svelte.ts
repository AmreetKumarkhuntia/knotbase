import { getContext, setContext } from 'svelte'
import { data } from './data.svelte'

export type Selection = { type: 'node' | 'edge'; id: number } | null

// Page UI state for one level, shared by its widgets through context; the data itself comes from the data store.
export class LevelState {
  selection = $state<Selection>(null)
  adding = $state(false)
  #id: () => number

  constructor(id: () => number) {
    this.#id = id
  }
  get id() {
    return this.#id()
  }
  get node() {
    return data.node(this.id)
  }
  get graph() {
    return data.graph(this.id)
  }
  select = (sel: Selection) => {
    if (sel?.type !== this.selection?.type || sel?.id !== this.selection?.id) this.selection = sel
  }
}

const KEY = Symbol('level')
export const provideLevel = (level: LevelState) => setContext(KEY, level)
export const useLevel = () => getContext<LevelState>(KEY)
