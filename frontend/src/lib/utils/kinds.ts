export const KINDS = ['frontend', 'backend', 'database', 'external', 'service', 'flow', 'step'] as const

// colour only ever means kind; an unknown kind falls back to muted
export const kindColor = (kind: string) => `var(--kind-${kind}, var(--muted))`
