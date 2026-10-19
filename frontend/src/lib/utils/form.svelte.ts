import type { Attachment } from 'svelte/attachments'

// Shared form behaviour: one in-flight submit at a time, Escape to cancel, focus on mount.
export function submitGuard(run: () => Promise<unknown>) {
  let busy = $state(false)
  return {
    get busy() {
      return busy
    },
    async submit(e: SubmitEvent) {
      e.preventDefault()
      if (busy) return
      busy = true
      try {
        await run()
      } finally {
        busy = false
      }
    },
  }
}

export const escape =
  (onEscape: () => void): Attachment<HTMLElement> =>
  (el) => {
    const h = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onEscape()
    }
    el.addEventListener('keydown', h)
    return () => el.removeEventListener('keydown', h)
  }

export const focus: Attachment<HTMLElement> = (el) => el.focus()
