// Shared error banner (TopBar renders it) and the one way mutations run: await, record the outcome, let the caller refetch.
export const ui = $state({ error: '' })

export async function act(p: Promise<unknown>): Promise<boolean> {
  try { await p; ui.error = ''; return true }
  catch (e) { ui.error = (e as Error).message; return false }
}
