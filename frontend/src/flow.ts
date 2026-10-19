export const KINDS = ['frontend', 'backend', 'database', 'external', 'service', 'flow', 'step'] as const

export const plural = (n: number, word: string) => `${n} ${word}${n === 1 ? '' : 's'}`
