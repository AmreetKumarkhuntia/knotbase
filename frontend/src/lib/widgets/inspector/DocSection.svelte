<script lang="ts">
  import { tick } from 'svelte'
  import { marked } from 'marked'
  import { nodes } from '$lib/services/nodes'
  import { focus } from '$lib/utils/form.svelte'
  import Section from '$lib/components/Section.svelte'
  import Button from '$lib/components/Button.svelte'

  let { id, doc, change }: { id: number; doc: string; change: (fn: () => Promise<unknown>) => Promise<boolean> } =
    $props()
  let editing = $state(false)
  let draft = $state('')
  let editBtn = $state<HTMLButtonElement>()

  // shortcut: an unsaved draft is dropped when another card is selected; keep a per-node draft Map when it bites
  function edit() {
    draft = doc
    editing = true
  }
  // the focused button unmounts; hand focus back
  async function close() {
    editing = false
    await tick()
    editBtn?.focus()
  }
  // stay in the editor on failure so the text isn't lost
  async function save(e: SubmitEvent) {
    e.preventDefault()
    if (await change(() => nodes.setDoc(id, draft))) close()
  }
</script>

<Section title="Doc" grow>
  {#snippet action()}
    {#if !editing && doc}<Button variant="ghost" bind:ref={editBtn} onclick={edit}>Edit</Button>{/if}
  {/snippet}
  {#if editing}
    <form class="stack" onsubmit={save}>
      <label class="sr-only" for="doc-md">Markdown</label>
      <textarea
        id="doc-md"
        {@attach focus}
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
</Section>

<style>
  .prose {
    max-width: 60ch;
    font-size: 14px;
    line-height: 1.6;
    overflow-wrap: anywhere;
  }
  .prose > :global(:first-child) {
    margin-top: 0;
  }
  .prose :global(h1),
  .prose :global(h2),
  .prose :global(h3) {
    margin: 16px 0 6px;
    font-size: 14px;
    font-weight: 700;
    line-height: 20px;
  }
  .prose :global(h1) {
    font-size: 17px;
    font-weight: 600;
    line-height: 24px;
  }
  .prose :global(p),
  .prose :global(ul),
  .prose :global(ol),
  .prose :global(pre) {
    margin: 0 0 10px;
  }
  .prose :global(ul),
  .prose :global(ol) {
    padding-left: 20px;
  }
  .prose :global(img) {
    max-width: 100%;
  }
  .prose :global(table) {
    display: block;
    overflow-x: auto;
  }
  .prose :global(code) {
    padding: 1px 5px;
    background: var(--code-bg);
    border: 1px solid var(--rule);
    border-radius: 3px;
    font: 12px var(--mono);
  }
  .prose :global(pre) {
    padding: 8px 12px;
    background: var(--code-bg);
    border: 1px solid var(--rule);
    border-radius: var(--radius);
    overflow-x: auto;
  }
  .prose :global(pre code) {
    padding: 0;
    border: 0;
    background: none;
  }
</style>
