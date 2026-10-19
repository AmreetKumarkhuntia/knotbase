<script lang="ts">
  import { KINDS } from '$lib/utils/kinds'
  import Button from '$lib/components/Button.svelte'
  import { escape, focus, submitGuard } from '$lib/utils/form.svelte'

  let {
    levelName,
    onAdd,
    onCancel,
  }: { levelName: string; onAdd: (name: string, kind: string) => Promise<boolean>; onCancel: () => void } = $props()
  let name = $state('')
  let kind = $state<string>('service')
  const form = submitGuard(async () => {
    if (name.trim() && (await onAdd(name.trim(), kind))) name = ''
  })
</script>

<form class="popover" onsubmit={form.submit} {@attach escape(onCancel)}>
  <span class="eyebrow">New node in {levelName}</span>
  <label class="field"
    >Name
    <input {@attach focus} bind:value={name} placeholder="e.g. Queue" required />
  </label>
  <label class="field"
    >Kind
    <select bind:value={kind}
      >{#each KINDS as k (k)}<option value={k}>{k}</option>{/each}</select
    >
  </label>
  <div class="row">
    <Button variant="primary" type="submit" disabled={form.busy}>Add</Button>
    <Button onclick={onCancel}>Cancel</Button>
  </div>
</form>

<style>
  .popover {
    position: absolute;
    right: 16px;
    top: 16px;
    z-index: 5;
    display: flex;
    flex-direction: column;
    gap: 10px;
    width: 300px;
    padding: 16px;
    background: var(--chrome);
    border: 1px solid var(--rule);
    border-radius: var(--radius);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }
</style>
