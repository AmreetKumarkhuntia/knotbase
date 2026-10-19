<script lang="ts">
  import Button from '../components/Button.svelte'

  let { onCreate, onCancel }: { onCreate: (name: string, desc: string) => Promise<boolean>; onCancel: () => void } = $props()
  let name = $state('')
  let desc = $state('')
  let busy = $state(false) // a double Enter must not create two systems

  async function submit(e: SubmitEvent) {
    e.preventDefault()
    if (busy || !name.trim()) return
    busy = true
    try { await onCreate(name.trim(), desc.trim()) } finally { busy = false }
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions: Escape closes the form from any field in it -->
<form class="panel" onsubmit={submit} onkeydown={(e) => { if (e.key === 'Escape') onCancel() }}>
  <span class="eyebrow">New system</span>
  <label class="field">Name
    <input {@attach (el) => el.focus()} bind:value={name} placeholder="e.g. Checkout platform" required />
  </label>
  <label class="field">One-line description
    <input bind:value={desc} placeholder="What this system is for" />
  </label>
  <div class="row">
    <Button variant="primary" type="submit" disabled={busy}>Create and open</Button>
    <Button onclick={onCancel}>Cancel</Button>
  </div>
</form>
