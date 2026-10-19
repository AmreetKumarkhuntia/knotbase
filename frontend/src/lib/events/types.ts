import type { ApiNode } from '$lib/services/api'

// The whole frontend-to-backend contract: one entry per event, mirroring the API table in CLAUDE.md.
// get:* events fill the data store and resolve to nothing; writes resolve to what the UI needs next.
export type System = { node: ApiNode; doc: string; inside: ApiNode[] }
export type Move = { id: number; x: number; y: number }

export type Events = {
  'get:node': { payload: { id: number }; result: void }
  'get:graph': { payload: { id: number }; result: void }
  'get:systems': { payload: Record<string, never>; result: void }
  'post:node': { payload: { parent_id: number; name: string; kind: string }; result: ApiNode | undefined }
  'patch:node': { payload: { id: number; patch: Partial<Pick<ApiNode, 'name' | 'kind' | 'doc'>> }; result: boolean }
  'patch:nodes': { payload: { parent_id: number; moves: Move[] }; result: boolean }
  'delete:node': { payload: { id: number; parent_id: number }; result: boolean }
  'delete:many': { payload: { parent_id: number; nodeIds: number[]; edgeIds: number[] }; result: boolean }
  'post:edge': { payload: { parent_id: number; source_id: number; target_id: number }; result: boolean }
  'patch:edge': { payload: { id: number; parent_id: number; label: string }; result: boolean }
  'delete:edge': { payload: { id: number; parent_id: number }; result: boolean }
  'post:item': { payload: { node_id: number; text: string }; result: boolean }
  'patch:item': { payload: { id: number; node_id: number; done: boolean }; result: boolean }
  'delete:item': { payload: { id: number; node_id: number }; result: boolean }
  'post:system': { payload: { name: string; desc: string }; result: number | undefined }
}
