<script lang="ts">
  import type { Item } from './api'

  let { items, onAdd, onToggle, onDelete }: {
    items: Item[]; onAdd: (text: string) => Promise<boolean>; onToggle: (item: Item) => void; onDelete: (id: number) => void
  } = $props()
  let text = $state('')

  async function add(e: SubmitEvent) {
    e.preventDefault()
    if (text.trim() && (await onAdd(text.trim()))) text = ''
  }
</script>

<section class="panel" aria-labelledby="items-h">
  <h2 id="items-h">Items</h2>
  {#if items.length}
    <ul class="items">
      {#each items as item (item.id)}
        <li class:done={item.done}>
          <label>
            <!-- no optimistic state: snap back to server truth, the refetch sets the new value -->
            <input type="checkbox" checked={item.done} onchange={(e) => { e.currentTarget.checked = item.done; onToggle(item) }} />
            <span>{item.text}</span>
          </label>
          <button class="icon" aria-label="Delete item: {item.text}" onclick={() => onDelete(item.id)}>×</button>
        </li>
      {/each}
    </ul>
  {:else}
    <p class="empty">No items yet. Add a task or a note to track here.</p>
  {/if}
  <form class="row" onsubmit={add}>
    <label class="field grow">New item
      <input bind:value={text} required />
    </label>
    <button class="primary">Add item</button>
  </form>
</section>
