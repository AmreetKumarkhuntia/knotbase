<script lang="ts">
  import { kindColor } from '$lib/utils/kinds'
  import { CARD } from '$lib/utils/layout'
  import { plural } from '$lib/utils/format'
  import type { System } from '$lib/types'

  let { system }: { system: System } = $props()
  const TW = 232,
    TH = 72 // the thumbnail box
  // the system's top level scaled into the thumbnail box, as the design draws it
  const thumb = $derived.by(() => {
    const inside = system.inside
    if (!inside.length) return []
    const minX = Math.min(...inside.map((n) => n.x)),
      minY = Math.min(...inside.map((n) => n.y))
    const spanX = Math.max(...inside.map((n) => n.x + CARD.w)) - minX,
      spanY = Math.max(...inside.map((n) => n.y + CARD.h)) - minY
    const sc = Math.min(TW / spanX, TH / spanY, 0.3)
    const ox = (TW - spanX * sc) / 2,
      oy = (TH - spanY * sc) / 2
    return inside.map((n) => ({
      id: n.id,
      kind: n.kind,
      l: ox + (n.x - minX) * sc,
      t: oy + (n.y - minY) * sc,
      w: CARD.w * sc,
      h: CARD.h * sc,
    }))
  })
</script>

<a href="/node/{system.node.id}" class="system-card">
  <span class="thumb" aria-hidden="true">
    {#each thumb as r (r.id)}
      <span
        class="mini"
        style:--kind={kindColor(r.kind)}
        style:left="{r.l}px"
        style:top="{r.t}px"
        style:width="{r.w}px"
        style:height="{r.h}px"
      ></span>
    {:else}
      <span class="mono muted">No nodes yet</span>
    {/each}
  </span>
  <span class="title">{system.node.name}</span>
  <span class="desc">{system.doc || 'No description yet.'}</span>
  <span class="mono muted foot">{plural(system.inside.length, 'node')}</span>
</a>

<style>
  .system-card {
    display: flex;
    flex-direction: column;
    gap: 12px;
    min-height: 212px;
    padding: 20px;
    background: var(--card);
    border: 1px solid var(--card-border);
    border-radius: var(--radius);
    color: var(--ink);
    text-align: left;
    text-decoration: none;
  }
  .thumb {
    position: relative;
    display: block;
    width: 232px;
    max-width: 100%;
    height: 72px;
    overflow: hidden;
  }
  .thumb > .mono {
    display: flex;
    align-items: center;
    height: 72px;
  }
  .mini {
    position: absolute;
    border: 1px solid var(--kind);
    border-radius: 2px;
    background: color-mix(in srgb, var(--kind) 22%, transparent);
  }
  .title {
    font-size: 18px;
    font-weight: 600;
    line-height: 24px;
    overflow-wrap: anywhere;
  }
  .desc {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    font-size: 13px;
    line-height: 1.5;
    color: var(--muted);
    overflow-wrap: anywhere;
  }
  .foot {
    margin-top: auto;
  }
</style>
