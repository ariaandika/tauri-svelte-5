# Tauri and Svelte 5 communication model

client server model of rust and svelte

looking to integrate with android ? check [this repo](https://github.com/ariaandika/kas-sosial)

## Command

a general request response model, good for invoking single command in rust

1. javascript send a request

```js
import { invoke } from "@tauri-apps/api/core";
let message  = "Send message to rust"
let response = invoke("custom_command", { message })
```

2. rust return a response

```rust
#[tauri::command]
pub fn custom_command(message: &str) -> String {
    println!("From JS: {message}");
    format!("{message}")
}
```

3. With svelte, we can use the `{#await}` tag

```svelte
<script>
  import { invoke } from "@tauri-apps/api/core";

  let promise = $state(Promise.resolve("None"));
  let message = $state("Send message to rust");

  function send() {
    promise = invoke("custom_command", { message })
  }
</script>

<div>
  {#await promise}
    Loading...
  {:then msg}
    `{msg}` from rust
  {:catch err}
    Error: {err?.message ?? "Fatal Error"}
  {/await}
</div>

<button onclick={send}>
  Send
</button>
```

see svelte example in `src/routes/Command.svelte`
see rust example in `src-tauri/src/command.rs`

## Channels

simmilar to request response, but the response is streamed down

good for user feedback in long running process that progress is made
instead of just a loading state

1. javascript send a request, and set hook on stream received

```js
import { Channel, invoke } from "@tauri-apps/api/core";

let progress = 0;

// channel only valid for one invoke
let channel = new Channel<ProgressEvent>()

channel.onmessage = ({ event, data }) => {
  if (event == "progress") {
    progress += data.chunk;
  }
}

let _response = await invoke("progress_channel", { channel });
```

2. rust stream data down

```rust
#[derive(serde::Serialize)]
pub enum ProgressEvent {
    Progress {
        chunk: i32,
    }
}

#[tauri::command]
pub async fn progress_channel(channel: tauri::ipc::Channel<ProgressEvent>) {
    for i in 1..=6 {
        let result = simulate_long_process(i);
        channel.send(ProgressEvent::Progress { chunk: result, }).unwrap();
    }

    channel.send(ProgressEvent::Progress { chunk: 4, }).unwrap();
}

async fn simulate_long_process(i: i32) -> i32 {
    use tokio::time::sleep;
    use std::time::Duration;

    sleep(Duration::from_millis((i * 100 + 200).try_into().unwrap())).await
    16
}
```

in svelte we can again, use `{#await}` tag, changing value
on stream, and its reactive

see svelte example in `src/routes/Channel.svelte`
see rust example in `src-tauri/src/channel.rs`

