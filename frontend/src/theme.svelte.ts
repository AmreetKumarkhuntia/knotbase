export type ThemeName = 'notebook' | 'console'

function stored(): ThemeName | null {
  try { const t = localStorage.getItem('theme'); return t === 'notebook' || t === 'console' ? t : null } catch { return null } // storage blocked
}
// Resolved to a concrete name here so app.css never duplicates a palette for prefers-color-scheme.
export const theme = $state<{ name: ThemeName }>({ name: stored() ?? (matchMedia('(prefers-color-scheme: dark)').matches ? 'console' : 'notebook') })
document.documentElement.dataset.theme = theme.name

export function toggleTheme() {
  theme.name = theme.name === 'console' ? 'notebook' : 'console'
  document.documentElement.dataset.theme = theme.name
  try { localStorage.setItem('theme', theme.name) } catch { /* storage blocked: not persisted */ }
}
