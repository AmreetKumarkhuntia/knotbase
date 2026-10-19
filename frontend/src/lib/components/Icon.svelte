<script module lang="ts">
  // every icon the app draws, on a 16-unit grid, stroked in the current colour; no inline <svg> anywhere else
  const PATHS = {
    plus: 'M8 3v10M3 8h10',
    arrow: 'M3 8h10M9 4l4 4-4 4',
    chevron: 'M6 3l5 5-5 5',
    x: 'M4 4l8 8M12 4l-8 8',
    sun: 'M11 8a3 3 0 1 1-6 0 3 3 0 0 1 6 0M8 1.5v2M8 12.5v2M1.5 8h2M12.5 8h2M3.4 3.4l1.4 1.4M11.2 11.2l1.4 1.4M3.4 12.6l1.4-1.4M11.2 4.8l1.4-1.4',
    moon: 'M13.5 9.5A5.5 5.5 0 0 1 6.5 2.5a5.5 5.5 0 1 0 7 7z',
    help: 'M14.25 8a6.25 6.25 0 1 1-12.5 0 6.25 6.25 0 0 1 12.5 0M6.2 6.3a1.9 1.9 0 0 1 3.7.4c0 1.2-1.9 1.4-1.9 2.6M8 11.6h.01',
  }
  export type IconName = keyof typeof PATHS
</script>

<script lang="ts">
  let { name, size = 16, class: cls }: { name: IconName; size?: number; class?: string } = $props()
</script>

<svg
  class={cls}
  width={size}
  height={size}
  viewBox="0 0 16 16"
  fill="none"
  stroke="currentColor"
  stroke-width="1.75"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"><path d={PATHS[name]}></path></svg
>
