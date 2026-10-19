import * as nodes from './nodes'
import * as edges from './edges'
import * as items from './items'
import * as systems from './systems'

// The central event listener: the only code that talks to the API. Called once from main.ts.
export function listen() {
  nodes.register()
  edges.register()
  items.register()
  systems.register()
}
