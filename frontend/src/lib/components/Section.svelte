<script lang="ts">
  import type { Snippet } from 'svelte'

  // One inspector section: eyebrow title, an optional action on the right, then its content.
  let {
    title,
    action,
    grow = false,
    children,
  }: { title: string; action?: Snippet; grow?: boolean; children: Snippet } = $props()
  const id = $derived(`section-${title.toLowerCase().replace(/\W+/g, '-')}`)
</script>

<section class:grow aria-labelledby={id}>
  <div class="row between">
    <h2 {id} class="eyebrow">{title}</h2>
    {@render action?.()}
  </div>
  {@render children()}
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 16px 20px;
    border-bottom: 1px solid var(--rule);
  }
  section.grow {
    flex: 1 1 auto;
    gap: 10px;
  }
  section:last-child {
    padding-bottom: 20px;
    border-bottom: 0;
  }
</style>
