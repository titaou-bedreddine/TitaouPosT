<script lang="ts">
  // Webcam capture: opens the camera, shows a live preview, snaps a photo
  // and hands back a data URL for the product image field.
  import { onMount, onDestroy } from 'svelte';

  export let onCapture: (dataUrl: string) => void = () => {};
  export let onClose: () => void = () => {};

  let video: HTMLVideoElement;
  let stream: MediaStream | null = null;
  let errorMsg = '';
  let ready = false;

  onMount(async () => {
    try {
      stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment', width: 640, height: 640 },
        audio: false,
      });
      if (video) {
        video.srcObject = stream;
        await video.play();
        ready = true;
      }
    } catch (e: any) {
      errorMsg = 'Camera unavailable: ' + (e?.message || e);
    }
  });

  onDestroy(() => {
    stream?.getTracks().forEach((t) => t.stop());
  });

  function snap() {
    if (!video || !ready) return;
    const canvas = document.createElement('canvas');
    // Square crop from the center.
    const side = Math.min(video.videoWidth, video.videoHeight);
    canvas.width = 480;
    canvas.height = 480;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    ctx.drawImage(
      video,
      (video.videoWidth - side) / 2,
      (video.videoHeight - side) / 2,
      side,
      side,
      0,
      0,
      480,
      480
    );
    onCapture(canvas.toDataURL('image/jpeg', 0.85));
    stream?.getTracks().forEach((t) => t.stop());
    onClose();
  }
</script>

<div class="fixed inset-0 z-[70] bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
  <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl w-full max-w-md p-5 space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="font-black text-sm text-pos-text">Capture Photo (التقاط صورة)</h3>
      <button
        type="button"
        on:click={() => {
          stream?.getTracks().forEach((t) => t.stop());
          onClose();
        }}
        class="p-1.5 text-pos-muted hover:text-pos-text rounded-lg cursor-pointer"
      >
        ✕
      </button>
    </div>

    {#if errorMsg}
      <div class="p-3 bg-rose-100 text-rose-700 text-xs font-bold rounded-lg">{errorMsg}</div>
    {/if}

    <div class="relative rounded-xl overflow-hidden bg-black aspect-square">
      <video bind:this={video} class="w-full h-full object-cover" muted playsinline></video>
      {#if !ready && !errorMsg}
        <div class="absolute inset-0 flex items-center justify-center text-white/70 text-xs font-bold">
          Starting camera...
        </div>
      {/if}
    </div>

    <button
      type="button"
      on:click={snap}
      disabled={!ready}
      class="w-full py-2.5 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white text-xs font-black rounded-xl cursor-pointer shadow-md"
    >
      Snap Photo (التقاط)
    </button>
  </div>
</div>
