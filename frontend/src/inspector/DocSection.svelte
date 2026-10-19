<script lang="ts">
  import { tick } from 'svelte'
  import { marked } from 'marked'

  let { doc, onSave }: { doc: string; onSave: (doc: string) => Promise<boolean> } = $props()
  let editing = $state(false)
  let draft = $state('')
  let editBtn = $state<HTMLButtonElement>()

  // shortcut: an unsaved draft is dropped on navigation; keep a per-node draft Map when it bites
  function edit() { draft = doc; editing = true }
  async function close() { editing = false; await tick(); editBtn?.focus() } // the focused button unmounts; hand focus back
  async function save() { if (await onSave(draft)) close() } // stay in the editor on failure so the text isn't lost
</script>

<section class="panel" aria-labelledby="doc-h">
  <div class="row">
    <h2 id="doc-h">Doc</h2>
    {#if !editing}<button bind:this={editBtn} onclick={edit}>Edit doc</button>{/if}
  </div>
  {#if editing}
    <label class="field">Markdown
      <textarea {@attach (el) => el.focus()} bind:value={draft} rows="14"></textarea>
    </label>
    <div class="row">
      <button class="primary" onclick={save}>Save doc</button>
      <button onclick={close}>Cancel</button>
    </div>
  {:else if doc}
    <!-- shortcut: {@html} renders unsanitised markdown, fine for a single-user local tool; add dompurify if docs ever come from another person. -->
    <div class="prose">{@html marked.parse(doc, { async: false })}</div>
  {:else}
    <p class="empty">No doc yet. Write down what this does and why.</p>
  {/if}
</section>
