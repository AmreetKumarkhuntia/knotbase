<script lang="ts">
  import { KINDS } from '$lib/utils/kinds'
  import Swatch from '$lib/components/Swatch.svelte'
</script>

<div class="legend" role="group" aria-label="Legend">
  {#each KINDS as k (k)}<span><Swatch kind={k} />{k}</span>{/each}
</div>

<style>
  .legend {
    position: absolute;
    left: 16px;
    bottom: 16px;
    z-index: 5;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 14px;
    max-width: calc(100% - 94px);
    padding: 10px 14px;
    background: var(--chrome);
    border: 1px solid var(--rule);
    border-radius: var(--radius);
    font: 11px/16px var(--mono);
    color: var(--muted);
  } /* leaves the controls column free */
  .legend span {
    display: flex;
    align-items: center;
    gap: 6px;
  }
</style>
