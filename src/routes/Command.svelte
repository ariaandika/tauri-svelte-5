<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let promise = $state(Promise.resolve("None"));
  let message = $state("Send message to rust");

  function send() {
    promise = invoke("custom_command", { message })
  }

</script>


<section class="relative p-8 w-full rounded-md bg-gray-800">
  <div class="mb-4">
    <kbd
      class="py-1 px-2 bg-gray-900 text-gray-400 rounded-md"
    >
        src/routes/Command.svelte
    </kbd>
  </div>

  <h1 class="mb-4 text-3xl font-bold"># Command</h1>

  <div class="mb-4">Call rust function from js</div>

  <label>
    <input bind:value={message} class="px-3 py-2 mb-4 bg-gray-900 text-gray-400 rounded-md">
    <button
      onclick={send}
      class="px-3 py-2 font-bold bg-gray-900 rounded-md hover:bg-gray-950 transition-colors"
    >
      Send
    </button>
  </label>

  <div class="mb-4">
    {#await promise}
      Loading...
    {:then msg}
      `{msg}` from rust
    {:catch err}
      Error: {err?.message ?? "Fatal Error"}
    {/await}
  </div>
</section>



