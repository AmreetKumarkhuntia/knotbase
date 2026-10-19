<script lang="ts">
  import type { Snippet } from 'svelte'

  let { title, text, children }: { title: string; text: string; children?: Snippet } = $props()
</script>

<div class="empty-card">
  <p class="title">{title}</p>
  <p class="muted">{text}</p>
  {@render children?.()}
</div>

<style>
  .empty-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    max-width: 36ch;
    padding: 24px 28px;
    background: var(--chrome);
    border: 1px solid var(--rule);
    border-radius: var(--radius);
    text-align: center;
    pointer-events: auto;
  }
  .title {
    font-size: 15px;
    font-weight: 600;
  }
</style>
