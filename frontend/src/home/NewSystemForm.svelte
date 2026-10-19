<script lang="ts">
  import Button from '../components/Button.svelte'

  let { onCreate, onCancel }: { onCreate: (name: string, desc: string) => Promise<boolean>; onCancel: () => void } = $props()
  let name = $state('')
  let desc = $state('')

  async function submit(e: SubmitEvent) {
    e.preventDefault()
    if (name.trim()) await onCreate(name.trim(), desc.trim())
  }
</script>

<form class="panel" onsubmit={submit}>
  <span class="eyebrow">New system</span>
  <label class="field">Name
    <input {@attach (el) => el.focus()} bind:value={name} placeholder="e.g. Checkout platform" required />
  </label>
  <label class="field">One-line description
    <input bind:value={desc} placeholder="What this system is for" />
  </label>
  <div class="row">
    <Button variant="primary" type="submit">Create and open</Button>
    <Button onclick={onCancel}>Cancel</Button>
  </div>
</form>
