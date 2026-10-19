<script lang="ts">
  import { bus } from '$lib/events/bus'
  import { useLevel } from '$lib/stores/level.svelte'
  import { data } from '$lib/stores/data.svelte'
  import { plural } from '$lib/utils/format'
  import Head from '$lib/components/Head.svelte'
  import Swatch from '$lib/components/Swatch.svelte'
  import NodeActions from './NodeActions.svelte'
  import InsideSection from './InsideSection.svelte'
  import DocSection from './DocSection.svelte'
  import ItemsSection from './ItemsSection.svelte'

  // The level itself or a selected card, both read from the data store; the head renders from the level graph before the detail lands.
  let { id }: { id: number } = $props()
  const level = useLevel()
  const isLevel = $derived(id === level.id)
  const detail = $derived(data.node(id))
  const children = $derived(data.graph(id))
  const status = $derived(data.status(`node:${id}`))
  const summary = $derived(isLevel ? level.node : level.graph?.nodes.find((n) => n.id === id))
  const meta = $derived.by(() => {
    const g = level.graph
    if (!g) return ''
    if (isLevel) return `${plural(g.nodes.length, 'node')} · ${plural(g.edges.length, 'connection')}`
    const conn = g.edges.filter((e) => e.source_id === id || e.target_id === id).length
    return `${children ? children.nodes.length + ' inside · ' : ''}${plural(conn, 'connection')} at this level`
  })
</script>

{#if summary}
  <!-- keyed by id so a rename draft, doc draft or confirm box never carries over to another node -->
  {#key id}
    <Head chip={isLevel ? 'this level' : 'selected'} onClear={isLevel ? undefined : () => level.select(null)}>
      {#snippet tag()}<Swatch kind={summary.kind} />{summary.kind}{/snippet}
      {#if !isLevel}<p class="sr-only" role="status">{summary.name} selected</p>{/if}
      <NodeActions {id} parentId={summary.parent_id} name={summary.name} {meta} {isLevel} />
    </Head>
    {#if detail && children}
      <!-- shortcut: picking a child of a selected card opens the card rather than selecting the child inside it; carry the child id in the nav:node payload when wanted. -->
      <InsideSection
        nodes={children.nodes}
        onPick={(cid) => (isLevel ? level.select({ type: 'node', id: cid }) : bus.emit('nav:node', { id }))}
      />
      <DocSection {id} doc={detail.doc} />
      <ItemsSection {id} items={detail.items} />
    {:else if typeof status === 'object'}
      <p class="note muted">{status.error}</p>
    {/if}
  {/key}
{/if}

<style>
  .note {
    padding: 16px 20px;
  }
</style>
