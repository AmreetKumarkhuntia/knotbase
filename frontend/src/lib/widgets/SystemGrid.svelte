<script lang="ts">
  import { bus } from '$lib/events/bus'
  import { navigate } from '$lib/stores/router.svelte'
  import { data } from '$lib/stores/data.svelte'
  import type { System } from '$lib/events/types'
  import SystemCard from '$lib/components/SystemCard.svelte'
  import NewSystemForm from '$lib/components/NewSystemForm.svelte'
  import Icon from '$lib/components/Icon.svelte'

  // The systems grid with its New system form; `adding` is bindable so the navbar action can open the form.
  let { adding = $bindable(false), onLoad }: { adding?: boolean; onLoad?: (systems: System[]) => void } = $props()
  const systems = $derived(data.systems())
  const status = $derived(data.status('systems'))
  $effect(() => {
    if (systems) onLoad?.(systems)
  })

  async function create(name: string, desc: string) {
    const id = await bus.emit('post:system', { name, desc })
    if (id) navigate(`/node/${id}`)
    return !!id
  }
</script>

{#if adding}<NewSystemForm onCreate={create} onCancel={() => (adding = false)} />{/if}
{#if systems}
  <div class="grid">
    {#each systems as s (s.node.id)}<SystemCard system={s} />{/each}
    <button class="new-card" onclick={() => (adding = true)}><Icon name="plus" size={24} />New system</button>
  </div>
{:else if typeof status === 'object'}
  <p class="empty">{status.error}</p>
{:else}
  <p class="empty">Loading…</p>
{/if}

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(min(280px, 100%), 1fr));
    gap: 16px;
  }
  .new-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    min-height: 212px;
    padding: 20px;
    background: transparent;
    border: 1px dashed var(--input-border);
    border-radius: var(--radius);
    color: var(--muted);
    font-weight: 500;
    cursor: pointer;
  }
</style>
