<script lang="ts">
  import type { Item } from '../api'
  import Button from '../components/Button.svelte'

  let { items, onAdd, onToggle, onDelete }: {
    items: Item[]; onAdd: (text: string) => Promise<boolean>; onToggle: (item: Item) => void; onDelete: (id: number) => void
  } = $props()
  let text = $state('')
  const done = $derived(items.filter(i => i.done).length)

  async function add(e: SubmitEvent) {
    e.preventDefault()
    if (text.trim() && (await onAdd(text.trim()))) text = ''
  }
</script>

<section class="section" aria-labelledby="items-h">
  <div class="row between">
    <h2 id="items-h" class="eyebrow">Items</h2>
    {#if items.length}<span class="mono muted">{done} / {items.length} done</span>{/if}
  </div>
  {#if items.length}
    <ul class="items">
      {#each items as item (item.id)}
        <li class:done={item.done}>
          <label>
            <!-- no optimistic state: snap back to server truth, the refetch sets the new value -->
            <input type="checkbox" checked={item.done} onchange={(e) => { e.currentTarget.checked = item.done; onToggle(item) }} />
            <span>{item.text}</span>
          </label>
          <Button variant="icon" aria-label="Delete item: {item.text}" onclick={() => onDelete(item.id)}>
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" aria-hidden="true"><path d="M4 4l8 8M12 4l-8 8"></path></svg>
          </Button>
        </li>
      {/each}
    </ul>
  {:else}
    <p class="muted">No items yet. Track a task or a note here.</p>
  {/if}
  <form class="row" onsubmit={add}>
    <label class="sr-only" for="new-item">New item</label>
    <input id="new-item" class="grow" bind:value={text} placeholder="Add an item…" required />
    <Button type="submit">Add</Button>
  </form>
</section>
