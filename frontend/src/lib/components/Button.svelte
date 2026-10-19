<script lang="ts">
  import type { Snippet } from 'svelte'
  import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements'

  // One button for the whole app; `href` renders it as a link that looks the same.
  let {
    variant = 'secondary',
    size = 'md',
    square = false,
    href,
    type = 'button',
    class: cls,
    ref = $bindable(),
    children,
    ...rest
  }: {
    variant?: 'primary' | 'secondary' | 'danger' | 'danger-fill' | 'ghost' | 'icon'
    size?: 'md' | 'sm'
    square?: boolean
    href?: string
    ref?: HTMLButtonElement
    children: Snippet
  } & HTMLButtonAttributes = $props()
</script>

{#if href}
  <a {href} class={['btn', variant, size, { square }, cls]} {...rest as unknown as HTMLAnchorAttributes}
    >{@render children()}</a
  >
{:else}
  <button bind:this={ref} {type} class={['btn', variant, size, { square }, cls]} {...rest}>{@render children()}</button>
{/if}

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    height: 40px;
    padding: 0 14px;
    border: 1px solid var(--input-border);
    border-radius: var(--radius);
    background: var(--secondary-bg);
    color: var(--ink);
    font-size: 14px;
    font-weight: 500;
    white-space: nowrap;
    text-decoration: none;
    cursor: pointer;
  }
  .btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
  .primary {
    border-color: var(--primary-bg);
    background: var(--primary-bg);
    color: var(--primary-fg);
    font-weight: 600;
  }
  .danger {
    color: var(--danger);
  }
  .danger-fill {
    border-color: var(--danger);
    background: var(--danger);
    color: var(--danger-fg);
    font-weight: 600;
  }
  .ghost {
    height: 32px;
    padding: 0 10px;
    border: 0;
    background: transparent;
    text-decoration: underline;
    text-underline-offset: 3px;
  }
  .icon {
    width: 44px;
    height: 44px;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--muted);
  }
  .square {
    width: 40px;
    padding: 0;
  }
  .sm {
    height: 24px;
    padding: 0 8px;
    background: transparent;
    font: 400 11px var(--mono);
  }
  .btn :global(svg) {
    flex: 0 0 auto;
  }
</style>
