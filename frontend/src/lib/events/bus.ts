import type { Events } from './types'

// A typed request/response event bus: exactly one handler per event name, and emit() resolves to that handler's result.
// Taps observe every emit (name, payload, result or error, duration) and can never affect it.
// Framework-free and app-free; the Events map in ./types.ts is the only place the app's types appear.
type Name = keyof Events
type Handler<K extends Name> = (payload: Events[K]['payload']) => Promise<Events[K]['result']>
export type TapEvent = { name: Name; payload: unknown; result?: unknown; error?: string; ms: number }
type Tap = (e: TapEvent) => void

const handlers = new Map<Name, Handler<Name>>()
const taps = new Set<Tap>()

function observe(e: TapEvent) {
  for (const tap of taps) {
    try {
      tap(e)
    } catch {
      /* a telemetry bug never breaks a handler */
    }
  }
}

export const bus = {
  on<K extends Name>(name: K, handler: Handler<K>) {
    if (handlers.has(name)) throw new Error(`event ${name} already has a handler`)
    handlers.set(name, handler as unknown as Handler<Name>)
  },
  tap(fn: Tap) {
    taps.add(fn)
    return () => {
      taps.delete(fn)
    }
  },
  async emit<K extends Name>(name: K, payload: Events[K]['payload']): Promise<Events[K]['result']> {
    const handler = handlers.get(name) as Handler<K> | undefined
    if (!handler) throw new Error(`no handler for event ${name}`)
    const t0 = performance.now()
    try {
      const result = await handler(payload)
      observe({ name, payload, result, ms: performance.now() - t0 })
      return result
    } catch (e) {
      observe({ name, payload, error: (e as Error).message, ms: performance.now() - t0 })
      throw e
    }
  },
}
