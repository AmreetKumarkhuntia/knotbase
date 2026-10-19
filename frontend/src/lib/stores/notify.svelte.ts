// The one error banner (TopBar renders it). Mutations go through act(); fetches go through load(), so a refetch never wipes a mutation's error.
export const notify = $state({ error: '' })
export const fail = (e: unknown) => {
  notify.error = (e as Error).message
}

export async function act(p: Promise<unknown>): Promise<boolean> {
  notify.error = ''
  try {
    await p
    return true
  } catch (e) {
    fail(e)
    return false
  }
}
export const load = (p: Promise<unknown>) => p.catch(fail)
