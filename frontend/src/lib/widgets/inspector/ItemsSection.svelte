<script lang="ts">
  import type { Item } from '$lib/services/api'
  import { nodes } from '$lib/services/nodes'
  import { submitGuard } from '$lib/utils/form.svelte'
  import Section from '$lib/components/Section.svelte'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'

  let { id, items, change }: { id: number; items: Item[]; change: (fn: () => Promise<unknown>) => Promise<boolean> } =
    $props()
  let text = $state('')
  const done = $derived(items.filter((i) => i.done).length)
  const form = submitGuard(async () => {
    if (text.trim() && (await change(() => nodes.addItem(id, text.trim())))) text = ''
  })
</script>

<Section title="Items">
  {#snippet action()}
    {#if items.length}<span class="mono muted">{done} / {items.length} done</span>{/if}
  {/snippet}
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
                change(() => nodes.toggleItem(item))
              }}
            />
            <span>{item.text}</span>
          </label>
          <Button
            variant="icon"
            aria-label="Delete item: {item.text}"
            onclick={() => change(() => nodes.removeItem(item.id))}><Icon name="x" /></Button
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
</Section>

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
