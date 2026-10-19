import * as telemetry from './telemetry'
import * as nav from './nav'
import * as ui from './ui'
import * as nodes from './nodes'
import * as edges from './edges'
import * as items from './items'
import * as systems from './systems'

// The central event listener: the only code that navigates, toggles the theme or talks to the API. Called once from main.ts.
export function listen() {
  telemetry.register()
  nav.register()
  ui.register()
  nodes.register()
  edges.register()
  items.register()
  systems.register()
}
