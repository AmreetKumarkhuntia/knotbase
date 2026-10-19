<script lang="ts">
  import { bus } from '$lib/events/bus'
  import { useLevel } from '$lib/stores/level.svelte'
  import { focus } from '$lib/utils/form.svelte'
  import Button from '$lib/components/Button.svelte'
  import Icon from '$lib/components/Icon.svelte'

  // Title, meta line and the actions on one node: rename inline, open, delete with an inline confirm.
  let {
    id,
    parentId,
    name,
    meta,
    isLevel,
  }: { id: number; parentId: number | null; name: string; meta: string; isLevel: boolean } = $props()
  const level = useLevel()
  let renaming = $state(false)
  let draft = $state('')
  let confirming = $state(false)

  async function rename(e: SubmitEvent) {
    e.preventDefault()
    if (draft.trim() && (await bus.emit('patch:node', { id, patch: { name: draft.trim() } }))) renaming = false
  }
  // deleting the level itself goes up a crumb; deleting a selected card clears the selection.
  // Decide before awaiting: the refresh inside the handler can clear the selection and re-key this inspector to the level.
  async function remove() {
    const atLevel = isLevel
    const up = level.node?.breadcrumb.at(-2)?.id ?? null
    if (!(await bus.emit('delete:node', { id, parent_id: parentId ?? 1 }))) return
    if (atLevel) bus.emit('nav:node', { id: up })
    else level.select(null)
  }
</script>

{#if renaming}
  <form class="row" onsubmit={rename}>
    <label class="sr-only" for="rename">Name</label>
    <input id="rename" class="grow" {@attach focus} bind:value={draft} required />
    <Button variant="primary" type="submit">Save</Button>
    <Button onclick={() => (renaming = false)}>Cancel</Button>
  </form>
{:else}
  <h1>{name}</h1>
{/if}
<p class="meta">{meta}</p>
{#if confirming}
  <div class="confirm">
    <span>Delete <b>{name}</b> and everything inside it? This can't be undone.</span>
    <div class="row">
      <Button variant="danger-fill" onclick={remove}>Delete</Button>
      <Button onclick={() => (confirming = false)}>Cancel</Button>
    </div>
  </div>
{:else}
  <div class="row">
    {#if !isLevel}
      <Button variant="primary" onclick={() => bus.emit('nav:node', { id })}>Open {name}<Icon name="arrow" /></Button>
    {/if}
    <Button
      onclick={() => {
        draft = name
        renaming = true
      }}>Rename</Button
    >
    <!-- the level can delete itself (the design hides this), else a system could never be removed -->
    <Button
      variant="danger"
      onclick={() => {
        confirming = true
        renaming = false
      }}>Delete</Button
    >
  </div>
{/if}

<style>
  .meta {
    font-size: 13px;
    color: var(--muted);
  }
  .confirm {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 12px 14px;
    border: 1px solid var(--input-border);
    border-radius: var(--radius);
    font-size: 13px;
  }
</style>
