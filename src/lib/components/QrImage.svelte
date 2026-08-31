<script lang="ts">
  // Offline QR image: renders locally via the qrcode library (data URL),
  // replacing the old api.qrserver.com network dependency.
  import { onMount } from 'svelte';
  import { entityQrDataUrl } from '../utils/printer';

  export let payload: string;
  export let size = 110;
  export let alt = 'QR';

  let src = '';

  $: if (payload) {
    // Regenerate whenever the payload changes.
    entityQrDataUrl(payload, size).then((url) => (src = url)).catch(() => {});
  }

  onMount(() => {
    entityQrDataUrl(payload, size).then((url) => (src = url)).catch(() => {});
  });
</script>

{#if src}
  <img {src} {alt} style="width:{size}px;height:{size}px;" />
{:else}
  <div style="width:{size}px;height:{size}px;" class="bg-slate-200 dark:bg-slate-700 rounded-lg animate-pulse"></div>
{/if}
