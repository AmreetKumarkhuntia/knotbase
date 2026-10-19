<script lang="ts">
  import { navigate } from '$lib/stores/router.svelte'
  import { useLevel } from '$lib/stores/level.svelte'
  import { NodeData } from '$lib/stores/node-data.svelte'
  import { plural } from '$lib/utils/format'
  import Head from '$lib/components/Head.svelte'
  import Swatch from '$lib/components/Swatch.svelte'
  import NodeActions from './NodeActions.svelte'
  import InsideSection from './InsideSection.svelte'
  import DocSection from './DocSection.svelte'
  import ItemsSection from './ItemsSection.svelte'

  // The level itself (data from the page) or a selected card (its own NodeData). The head renders from the level graph before the detail lands.
  let { id }: { id: number } = $props()
  const level = useLevel()
  const isLevel = $derived(id === level.id)
  const picked = new NodeData(() => (isLevel ? null : id))
  const data = $derived(isLevel ? level.level.data : picked.data)
  const summary = $derived(isLevel ? level.node : level.graph?.nodes.find((n) => n.id === id))
  const meta = $derived.by(() => {
    const g = level.graph
    if (!g) return ''
    if (isLevel) return `${plural(g.nodes.length, 'node')} · ${plural(g.edges.length, 'connection')}`
    const conn = g.edges.filter((e) => e.source_id === id || e.target_id === id).length
    return `${data ? data.graph.nodes.length + ' inside · ' : ''}${plural(conn, 'connection')} at this level`
  })
  const change = (fn: () => Promise<unknown>) => level.change(fn, isLevel ? undefined : picked)
</script>

{#if summary}
  <!-- keyed by id so a rename draft, doc draft or confirm box never carries over to another node -->
  {#key id}
    <Head chip={isLevel ? 'this level' : 'selected'} onClear={isLevel ? undefined : () => level.select(null)}>
      {#snippet tag()}<Swatch kind={summary.kind} />{summary.kind}{/snippet}
      {#if !isLevel}<p class="sr-only" role="status">{summary.name} selected</p>{/if}
      <NodeActions {id} name={summary.name} {meta} {isLevel} {change} />
    </Head>
    {#if data}
      <!-- shortcut: picking a child of a selected card opens the card rather than selecting the child inside it; carry the child id through navigate() when wanted. -->
      <InsideSection
        nodes={data.graph.nodes}
        onPick={(cid) => (isLevel ? level.select({ type: 'node', id: cid }) : navigate(`/node/${id}`))}
      />
      <DocSection {id} doc={data.node.doc} {change} />
      <ItemsSection {id} items={data.node.items} {change} />
    {:else if picked.error}
      <p class="note muted">{picked.error}</p>
    {/if}
  {/key}
{/if}

<style>
  .note {
    padding: 16px 20px;
  }
</style>
