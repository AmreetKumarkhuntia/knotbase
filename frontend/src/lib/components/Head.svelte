<script lang="ts">
  import type { Snippet } from 'svelte'
  import Button from './Button.svelte'

  // The inspector head shell: a tag line saying what is inspected, a chip, Clear, then whatever the inspector puts under it.
  let { tag, chip, onClear, children }: { tag: Snippet; chip?: string; onClear?: () => void; children: Snippet } =
    $props()
</script>

<div class="head">
  <span class="tagline">
    {@render tag()}<span class="spacer"></span>
    {#if chip}<span>{chip}</span>{/if}
    {#if onClear}<Button size="sm" onclick={onClear}>Clear</Button>{/if}
  </span>
  {@render children()}
</div>

<style>
  .head {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 20px 20px 16px;
    border-bottom: 1px solid var(--rule);
  }
  .tagline {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 24px;
    font: 11px/16px var(--mono);
    color: var(--muted);
  }
</style>
