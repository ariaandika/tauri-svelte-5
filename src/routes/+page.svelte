<script lang="ts">
  import Command from "./Command.svelte";
  import Channel from "./Channel.svelte";
</script>


<section class="container mx-auto w-full h-screen p-16">
  <h1 class="mb-4 w-full text-center text-4xl font-bold text-white">
    Tauri Dev
  </h1>

  <div class="grid gap-4">
    <Command/>
    <Channel/>
  </div>
</section>

