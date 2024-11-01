<script lang="ts">
  import { Channel, invoke } from "@tauri-apps/api/core";

  type ProgressEvent =
  | {
      event: 'progress';
      data: {
        chunk: number,
      }
    }

  let promise = $state(Promise.resolve());
  let progress = $state(100);
  let isPending = $derived(progress != 100);

  $inspect(progress);

  function start() {
    // channel only valid for one invoke
    let channel = new Channel<ProgressEvent>()
    channel.onmessage = (data) => {
      if (data.event == "progress") {
        progress += data.data.chunk;
      }
    }
    progress = 0;
    promise = invoke("progress_channel", { channel });
  }
</script>

<section class="relative p-8 w-full rounded-md bg-gray-800">
  <div class="mb-4">
    <kbd
      class="py-1 px-2 bg-gray-900 text-gray-400 rounded-md"
    >
        src/routes/Channel.svelte
    </kbd>
  </div>

  <h1 class="mb-4 text-3xl font-bold"># Command + Channel</h1>

  <div class="mb-4">Call rust function then stream the result</div>

  <div class="p-1 w-full h-4 bg-gray-900 rounded-md">
    <div
      class="h-full bg-blue-500 rounded-md"
      style:width="{progress}%"
    >&ThickSpace;</div>
  </div>

  <button
    onclick={start}
    disabled={isPending}
    class="px-3 py-2 font-bold bg-gray-900 rounded-md hover:bg-gray-950 transition-colors"
  >
    {#await promise}
      Loading...
    {:then}
      Start
    {:catch err}
      Error: {err?.message ?? err}
    {/await}
  </button>

</section>
