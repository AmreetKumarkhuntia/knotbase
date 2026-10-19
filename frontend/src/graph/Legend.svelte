<script lang="ts">
  import { KINDS } from '../flow'
  import Swatch from '../components/Swatch.svelte'
</script>

<div class="legend" aria-label="Legend">
  {#each KINDS as k (k)}<span><Swatch kind={k} />{k}</span>{/each}
</div>
