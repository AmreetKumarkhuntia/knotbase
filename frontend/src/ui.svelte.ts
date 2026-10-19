// Shared error banner (TopBar renders it). Mutations go through act(); loads go through load(), so a refetch never wipes a mutation's error.
export const ui = $state({ error: '' })
export const fail = (e: unknown) => { ui.error = (e as Error).message }

export async function act(p: Promise<unknown>): Promise<boolean> {
  ui.error = ''
  try { await p; return true } catch (e) { fail(e); return false }
}
export const load = (p: Promise<unknown>) => p.catch(fail)
