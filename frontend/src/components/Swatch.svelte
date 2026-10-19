<script lang="ts">
  let { kind, size = 8 }: { kind: string; size?: number } = $props()
</script>

<!-- colour only ever means kind; an unknown kind falls back to muted -->
<span class="swatch" aria-hidden="true" style:--kind="var(--kind-{kind}, var(--muted))" style:width="{size}px" style:height="{size}px"></span>
