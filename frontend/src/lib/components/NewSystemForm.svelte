<script lang="ts">
  import Button from '$lib/components/Button.svelte'
  import { escape, focus, submitGuard } from '$lib/utils/form.svelte'

  let { onCreate, onCancel }: { onCreate: (name: string, desc: string) => Promise<boolean>; onCancel: () => void } =
    $props()
  let name = $state('')
  let desc = $state('')
  const form = submitGuard(async () => {
    if (name.trim()) await onCreate(name.trim(), desc.trim())
  })
</script>

<form class="panel" onsubmit={form.submit} {@attach escape(onCancel)}>
  <span class="eyebrow">New system</span>
  <label class="field"
    >Name
    <input {@attach focus} bind:value={name} placeholder="e.g. Checkout platform" required />
  </label>
  <label class="field"
    >One-line description
    <input bind:value={desc} placeholder="What this system is for" />
  </label>
  <div class="row">
    <Button variant="primary" type="submit" disabled={form.busy}>Create and open</Button>
    <Button onclick={onCancel}>Cancel</Button>
  </div>
</form>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 16px;
    background: var(--chrome);
    border: 1px solid var(--rule);
    border-radius: var(--radius);
  }
</style>
