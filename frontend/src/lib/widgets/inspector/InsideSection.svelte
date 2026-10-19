<script lang="ts">
  import type { ApiNode } from '$lib/services/api'
  import Swatch from '$lib/components/Swatch.svelte'
  import Icon from '$lib/components/Icon.svelte'

  let { nodes, onPick }: { nodes: ApiNode[]; onPick: (id: number) => void } = $props()
</script>

<section class="section" aria-labelledby="inside-h">
  <h2 id="inside-h" class="eyebrow">Inside</h2>
  {#each nodes as n (n.id)}
    <button class="rowbtn" onclick={() => onPick(n.id)}>
      <Swatch kind={n.kind} size={10} />
      <span class="grow">{n.name}</span>
      <Icon name="chevron" />
    </button>
  {:else}
    <p class="muted">Nothing inside yet.</p>
  {/each}
</section>

<style>
  .rowbtn {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 44px;
    margin: 0 -10px;
    padding: 0 10px;
    border: 0;
    border-radius: var(--radius);
    background: transparent;
    color: var(--ink);
    font-weight: 500;
    text-align: left;
    cursor: pointer;
  }
  .rowbtn:hover {
    background: var(--ground);
  }
  .rowbtn :global(.swatch) {
    border-radius: 3px;
  }
</style>
