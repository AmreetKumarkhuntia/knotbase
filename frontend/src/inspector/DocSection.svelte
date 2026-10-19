<script lang="ts">
  import { tick } from 'svelte'
  import { marked } from 'marked'
  import Button from '../components/Button.svelte'

  let { doc, onSave }: { doc: string; onSave: (doc: string) => Promise<boolean> } = $props()
  let editing = $state(false)
  let draft = $state('')
  let editBtn = $state<HTMLButtonElement>()

  // shortcut: an unsaved draft is dropped on navigation; keep a per-node draft Map when it bites
  function edit() {
    draft = doc
    editing = true
  }
  async function close() {
    editing = false
    await tick()
    editBtn?.focus()
  } // the focused button unmounts; hand focus back
  async function save(e: SubmitEvent) {
    e.preventDefault()
    if (await onSave(draft)) close()
  } // stay in the editor on failure so the text isn't lost
</script>

<section class="section doc" aria-labelledby="doc-h">
  <div class="row between">
    <h2 id="doc-h" class="eyebrow">Doc</h2>
    {#if !editing && doc}<Button variant="ghost" bind:ref={editBtn} onclick={edit}>Edit</Button>{/if}
  </div>
  {#if editing}
    <form class="stack" onsubmit={save}>
      <label class="sr-only" for="doc-md">Markdown</label>
      <textarea
        id="doc-md"
        {@attach (el) => el.focus()}
        bind:value={draft}
        rows="12"
        placeholder="# Heading, paragraphs, - bullets, `code`"></textarea>
      <div class="row">
        <Button variant="primary" type="submit">Save doc</Button>
        <Button onclick={close}>Cancel</Button>
      </div>
    </form>
  {:else if doc}
    <!-- shortcut: {@html} renders unsanitised markdown, fine for a single-user local tool; add dompurify if docs ever come from another person. -->
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    <div class="prose">{@html marked.parse(doc, { async: false })}</div>
  {:else}
    <p class="muted">No doc yet. Write down what this does and why it exists.</p>
    <Button class="self-start" bind:ref={editBtn} onclick={edit}>Write doc</Button>
  {/if}
</section>
