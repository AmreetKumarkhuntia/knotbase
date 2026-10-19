<script lang="ts">
  import { KINDS } from '../flow'
  import Button from '../components/Button.svelte'

  let {
    levelName,
    onAdd,
    onCancel,
  }: { levelName: string; onAdd: (name: string, kind: string) => Promise<boolean>; onCancel: () => void } = $props()
  let name = $state('')
  let kind = $state<string>('service')
  let busy = $state(false) // a double Enter must not create two nodes

  async function submit(e: SubmitEvent) {
    e.preventDefault()
    if (busy || !name.trim()) return
    busy = true
    try {
      if (await onAdd(name.trim(), kind)) name = ''
    } finally {
      busy = false
    }
  }
</script>

<!-- Escape closes the popover from any field in it -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<form
  class="popover"
  onsubmit={submit}
  onkeydown={(e) => {
    if (e.key === 'Escape') onCancel()
  }}
>
  <span class="eyebrow">New node in {levelName}</span>
  <label class="field"
    >Name
    <input {@attach (el) => el.focus()} bind:value={name} placeholder="e.g. Queue" required />
  </label>
  <label class="field"
    >Kind
    <select bind:value={kind}
      >{#each KINDS as k (k)}<option value={k}>{k}</option>{/each}</select
    >
  </label>
  <div class="row">
    <Button variant="primary" type="submit" disabled={busy}>Add</Button>
    <Button onclick={onCancel}>Cancel</Button>
  </div>
</form>
