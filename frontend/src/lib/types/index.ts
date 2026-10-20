// The data shapes. The first five are the API bodies and mirror backend/src/types/ field for field; System and Move are client-side.
export type ApiNode = {
  id: number
  parent_id: number | null
  name: string
  kind: string
  doc: string
  x: number
  y: number
  created_at: string
  updated_at: string
}
export type ApiEdge = {
  id: number
  parent_id: number
  source_id: number
  target_id: number
  label: string
  created_at: string
  updated_at: string
}
export type Item = { id: number; node_id: number; text: string; done: boolean; created_at: string; updated_at: string }
export type Graph = { nodes: ApiNode[]; edges: ApiEdge[] }
export type NodeDetail = ApiNode & { items: Item[]; breadcrumb: { id: number; name: string }[] }
export type System = { node: ApiNode; doc: string; inside: ApiNode[] }
export type Move = { id: number; x: number; y: number }
