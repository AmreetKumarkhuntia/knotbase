<script lang="ts">
  import { kindColor } from '$lib/utils/kinds'

  let { kind, size = 8 }: { kind: string; size?: number } = $props()
</script>

<span class="swatch" aria-hidden="true" style:--kind={kindColor(kind)} style:width="{size}px" style:height="{size}px"
></span>

<style>
  .swatch {
    display: inline-block;
    flex: 0 0 auto;
    border-radius: 2px;
    background: var(--kind);
  }
</style>
