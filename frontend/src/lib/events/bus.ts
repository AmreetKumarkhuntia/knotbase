import type { Events } from './types'

// A typed request/response event bus: exactly one handler per event name, and emit() returns that handler's promise.
// Framework-free and app-free; the Events map in ./types.ts is the only place the app's types appear.
type Name = keyof Events
type Handler<K extends Name> = (payload: Events[K]['payload']) => Promise<Events[K]['result']>

const handlers = new Map<Name, Handler<Name>>()

export const bus = {
  on<K extends Name>(name: K, handler: Handler<K>) {
    if (handlers.has(name)) throw new Error(`event ${name} already has a handler`)
    handlers.set(name, handler as unknown as Handler<Name>)
  },
  emit<K extends Name>(name: K, payload: Events[K]['payload']): Promise<Events[K]['result']> {
    const handler = handlers.get(name) as Handler<K> | undefined
    if (!handler) throw new Error(`no handler for event ${name}`)
    if (import.meta.env.DEV) console.debug('[event]', name, payload)
    return handler(payload)
  },
}
