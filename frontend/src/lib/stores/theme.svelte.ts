export type ThemeName = 'notebook' | 'console'

// index.html resolves the stored or OS theme before the first paint; this mirrors it into state.
export const theme = $state<{ name: ThemeName }>({
  name: document.documentElement.dataset.theme === 'console' ? 'console' : 'notebook',
})

export function toggleTheme() {
  theme.name = theme.name === 'console' ? 'notebook' : 'console'
  document.documentElement.dataset.theme = theme.name
  try {
    localStorage.setItem('theme', theme.name)
  } catch {
    /* storage blocked: not persisted */
  }
}
