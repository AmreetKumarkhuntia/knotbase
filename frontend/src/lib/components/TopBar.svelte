<script module lang="ts">
  import type { IconName } from './Icon.svelte'
  export type TopBarAction = { label: string; icon?: IconName; disabled?: boolean; onClick: () => void }
</script>

<script lang="ts">
  import Button from './Button.svelte'
  import Icon from './Icon.svelte'

  // The navbar. Crumbs come from the API and start at root 1, which renders as "Home"; no crumbs at all means the Home page.
  let {
    crumbs,
    stats = '',
    action,
    dark,
    onToggleTheme,
    error = '',
  }: {
    crumbs?: { id: number; name: string }[]
    stats?: string
    action?: TopBarAction
    dark: boolean
    onToggleTheme: () => void
    error?: string
  } = $props()
</script>

<header class="topbar">
  <a href="/" class="wordmark">knotbase</a>
  <nav class="crumbs" aria-label="Breadcrumb">
    <span class="sep" aria-hidden="true">/</span>
    {#if crumbs}
      <a href="/">Home</a>
      {#each crumbs as c, i (c.id)}
        {#if c.id !== 1}
          <span class="sep" aria-hidden="true">/</span>
          {#if i === crumbs.length - 1}<span aria-current="page">{c.name}</span>{:else}<a href="/node/{c.id}"
              >{c.name}</a
            >{/if}
        {/if}
      {/each}
    {:else}
      <span aria-current="page">Home</span>
    {/if}
  </nav>
  <span class="spacer"></span>
  {#if stats}<span class="stats">{stats}</span>{/if}
  <Button
    square
    onclick={onToggleTheme}
    aria-label={dark ? 'Switch to light theme (Field notebook)' : 'Switch to dark theme (Night console)'}
  >
    <Icon name={dark ? 'sun' : 'moon'} />
  </Button>
  {#if action}
    <Button variant="primary" disabled={action.disabled} onclick={action.onClick}>
      {#if action.icon}<Icon name={action.icon} />{/if}{action.label}
    </Button>
  {/if}
</header>
{#if error}<p class="error" role="alert">{error}</p>{/if}

<style>
  .topbar {
    flex: none;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
    min-height: 56px;
    padding: 8px 16px;
    background: var(--chrome);
    border-bottom: 1px solid var(--rule);
  }
  .wordmark {
    display: inline-flex;
    align-items: center;
    height: 40px;
    padding: 0 6px;
    font: 700 16px var(--wordmark-font);
    letter-spacing: -0.01em;
    text-decoration: none;
  }
  .crumbs {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 4px;
  }
  .sep {
    padding: 0 4px;
    color: var(--muted);
  }
  .crumbs a,
  .crumbs [aria-current] {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    height: 40px;
    max-width: 24ch;
    padding: 0 8px;
    border-radius: var(--radius);
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    text-decoration: none;
  }
  .crumbs a {
    color: var(--muted);
  }
  .crumbs [aria-current] {
    font-weight: 600;
  }
  .stats {
    padding-right: 8px;
    font: 12px var(--mono);
    color: var(--muted);
  }
  .error {
    position: sticky;
    top: 0;
    z-index: 10;
    flex: none;
    padding: 8px 16px;
    background: color-mix(in srgb, var(--danger) 15%, var(--chrome));
    color: var(--danger);
    border-bottom: 1px solid var(--danger);
  }
</style>
