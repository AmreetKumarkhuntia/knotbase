// The one error banner (the layout renders it). Every mutation handler runs its API call through act().
export const notify = $state({ error: '' })

export async function act(p: Promise<unknown>): Promise<boolean> {
  notify.error = ''
  try {
    await p
    return true
  } catch (e) {
    notify.error = (e as Error).message
    return false
  }
}
