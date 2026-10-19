<script lang="ts">
  import type { Node } from '../api'
  import Swatch from '../components/Swatch.svelte'

  let { nodes, onPick }: { nodes: Node[]; onPick: (id: number) => void } = $props()
</script>

<section class="section" aria-labelledby="inside-h">
  <h2 id="inside-h" class="eyebrow">Inside</h2>
  {#each nodes as n (n.id)}
    <button class="rowbtn" onclick={() => onPick(n.id)}>
      <Swatch kind={n.kind} size={10} />
      <span class="grow">{n.name}</span>
      <svg
        width="16"
        height="16"
        viewBox="0 0 16 16"
        fill="none"
        stroke="currentColor"
        stroke-width="1.75"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"><path d="M6 3l5 5-5 5"></path></svg
      >
    </button>
  {:else}
    <p class="muted">Nothing inside yet.</p>
  {/each}
</section>
