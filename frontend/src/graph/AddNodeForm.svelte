<script lang="ts">
  import { KINDS } from '../flow'
  import Button from '../components/Button.svelte'

  let { levelName, onAdd, onCancel }: { levelName: string; onAdd: (name: string, kind: string) => Promise<boolean>; onCancel: () => void } = $props()
  let name = $state('')
  let kind = $state<string>('service')

  async function submit(e: SubmitEvent) {
    e.preventDefault()
    if (name.trim() && (await onAdd(name.trim(), kind))) name = ''
  }
</script>

<form class="popover" onsubmit={submit}>
  <span class="eyebrow">New node in {levelName}</span>
  <label class="field">Name
    <input {@attach (el) => el.focus()} bind:value={name} placeholder="e.g. Queue" required />
  </label>
  <label class="field">Kind
    <select bind:value={kind}>{#each KINDS as k (k)}<option value={k}>{k}</option>{/each}</select>
  </label>
  <div class="row">
    <Button variant="primary" type="submit">Add</Button>
    <Button onclick={onCancel}>Cancel</Button>
  </div>
</form>
