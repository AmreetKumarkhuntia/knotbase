<script lang="ts">
  import type { Item } from '$lib/services/api'
  import { bus } from '$lib/events/bus'
  import { submitGuard } from '$lib/utils/form.svelte'
  import Section from '$lib/components/Section.svelte'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'

  let { id, items }: { id: number; items: Item[] } = $props()
  let text = $state('')
  const done = $derived(items.filter((i) => i.done).length)
  const form = submitGuard(async () => {
    if (text.trim() && (await bus.emit('post:item', { node_id: id, text: text.trim() }))) text = ''
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
            <!-- no optimistic state: snap back to server truth, the refresh sets the new value -->
            <input
              type="checkbox"
              checked={item.done}
              onchange={(e) => {
                e.currentTarget.checked = item.done
                bus.emit('patch:item', { id: item.id, node_id: id, done: !item.done })
              }}
            />
            <span>{item.text}</span>
          </label>
          <Button
            variant="icon"
            aria-label="Delete item: {item.text}"
            onclick={() => bus.emit('delete:item', { id: item.id, node_id: id })}><Icon name="x" /></Button
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
