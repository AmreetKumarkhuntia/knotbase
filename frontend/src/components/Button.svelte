<script lang="ts">
  import type { Snippet } from 'svelte'
  import type { HTMLButtonAttributes } from 'svelte/elements'

  let {
    variant = 'secondary',
    type = 'button',
    class: cls,
    ref = $bindable(),
    children,
    ...rest
  }: {
    variant?: 'primary' | 'secondary' | 'danger' | 'danger-fill' | 'ghost' | 'icon'
    ref?: HTMLButtonElement
    children: Snippet
  } & HTMLButtonAttributes = $props()
</script>

<button bind:this={ref} {type} class={['btn', variant, cls]} {...rest}>{@render children()}</button>
