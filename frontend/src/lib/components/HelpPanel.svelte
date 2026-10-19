<div class="help">
  <span class="eyebrow">How to use</span>
  <span>Click a card to inspect it. Double-click, or press Open, to go inside. Drag a card to move it.</span>
  <span
    >Drag from a card's right-hand dot onto another card to connect them. Click a connection to label or remove it.</span
  >
  <span>Select a card or a connection and press Backspace to delete it.</span>
</div>

<style>
  /* bottom: the four 44px controls plus their margins; grows if a control is added */
  .help {
    position: absolute;
    right: 16px;
    bottom: 204px;
    z-index: 5;
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 300px;
    padding: 14px 16px;
    background: var(--chrome);
    border: 1px solid var(--rule);
    border-radius: var(--radius);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    font-size: 13px;
    line-height: 1.5;
  }
</style>
