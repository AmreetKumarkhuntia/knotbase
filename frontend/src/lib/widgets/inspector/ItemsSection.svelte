<script lang="ts">
  import type { Item } from '$lib/services/api'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'
  import { submitGuard } from '$lib/utils/form.svelte'

  let {
    items,
    onAdd,
    onToggle,
    onDelete,
  }: {
    items: Item[]
    onAdd: (text: string) => Promise<boolean>
    onToggle: (item: Item) => void
    onDelete: (id: number) => void
  } = $props()
  let text = $state('')
  const done = $derived(items.filter((i) => i.done).length)
  const form = submitGuard(async () => {
    if (text.trim() && (await onAdd(text.trim()))) text = ''
  })
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
            <input
              type="checkbox"
              checked={item.done}
              onchange={(e) => {
                e.currentTarget.checked = item.done
                onToggle(item)
              }}
            />
            <span>{item.text}</span>
          </label>
          <Button variant="icon" aria-label="Delete item: {item.text}" onclick={() => onDelete(item.id)}
            ><Icon name="x" /></Button
          >
        </li>
      {/each}
    </ul>
  {:else}
    <p class="muted">No items yet. Track a task or a note here.</p>
  {/if}
  <form class="row" onsubmit={form.submit}>
    <label class="sr-only" for="new-item">New item</label>
    <input id="new-item" class="grow" bind:value={text} placeholder="Add an item…" required />
    <Button type="submit" disabled={form.busy}>Add</Button>
  </form>
</section>

<style>
  .items {
    display: flex;
    flex-direction: column;
    margin: 0;
    padding: 0;
    list-style: none;
  }
  li {
    display: flex;
    align-items: center;
    gap: 10px;
    min-height: 44px;
  }
  label {
    display: flex;
    flex: 1;
    align-items: center;
    gap: 10px;
    cursor: pointer;
  }
  .done span {
    color: var(--muted);
    text-decoration: line-through;
  }
  li :global(.btn) {
    margin-right: -14px;
  }
</style>
