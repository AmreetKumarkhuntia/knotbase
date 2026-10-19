<script lang="ts">
  import type { Node } from '../api'
  import Link from '../components/Link.svelte'
  import { plural } from '../flow'

  let { node, doc, inside }: { node: Node; doc: string; inside: Node[] } = $props()
  // shortcut: mirrors the .card and .thumb sizes in app.css; derive from CSS vars if cards ever become resizable
  const CW = 216, CH = 76, TW = 232, TH = 72
  // the system's top level scaled into the thumbnail box, as the prototype draws it
  const thumb = $derived.by(() => {
    if (!inside.length) return []
    const minX = Math.min(...inside.map(n => n.x)), minY = Math.min(...inside.map(n => n.y))
    const spanX = Math.max(...inside.map(n => n.x + CW)) - minX, spanY = Math.max(...inside.map(n => n.y + CH)) - minY
    const sc = Math.min(TW / spanX, TH / spanY, 0.3)
    const ox = (TW - spanX * sc) / 2, oy = (TH - spanY * sc) / 2
    return inside.map(n => ({ id: n.id, kind: n.kind, l: ox + (n.x - minX) * sc, t: oy + (n.y - minY) * sc, w: CW * sc, h: CH * sc }))
  })
</script>

<Link href="/node/{node.id}" class="system-card">
  <span class="thumb" aria-hidden="true">
    {#each thumb as r (r.id)}
      <span class="mini" style:--kind="var(--kind-{r.kind}, var(--muted))" style:left="{r.l}px" style:top="{r.t}px" style:width="{r.w}px" style:height="{r.h}px"></span>
    {:else}
      <span class="mono muted">No nodes yet</span>
    {/each}
  </span>
  <span class="title">{node.name}</span>
  <span class="desc">{doc || 'No description yet.'}</span>
  <span class="mono muted foot">{plural(inside.length, 'node')}</span>
</Link>
