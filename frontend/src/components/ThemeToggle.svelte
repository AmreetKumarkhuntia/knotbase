<script lang="ts">
  import Button from './Button.svelte'
  import { theme, toggleTheme } from '../theme.svelte'

  const dark = $derived(theme.name === 'console')
</script>

<Button class="square" onclick={toggleTheme} aria-label={dark ? 'Switch to light theme (Field notebook)' : 'Switch to dark theme (Night console)'}>
  {#if dark}
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="8" cy="8" r="3"></circle><path d="M8 1.5v2M8 12.5v2M1.5 8h2M12.5 8h2M3.4 3.4l1.4 1.4M11.2 11.2l1.4 1.4M3.4 12.6l1.4-1.4M11.2 4.8l1.4-1.4"></path></svg>
  {:else}
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M13.5 9.5A5.5 5.5 0 0 1 6.5 2.5a5.5 5.5 0 1 0 7 7z"></path></svg>
  {/if}
</Button>
