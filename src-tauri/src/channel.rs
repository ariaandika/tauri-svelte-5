use tokio::time::sleep;
use std::time::Duration;

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum ProgressEvent {
    Progress {
        chunk: i32,
    }
}

#[tauri::command]
pub async fn progress_channel(channel: tauri::ipc::Channel<ProgressEvent>) {
    for i in 1..=6 {
        channel.send(ProgressEvent::Progress { chunk: 16, }).unwrap();
        sleep(Duration::from_millis((i * 100 + 200).try_into().unwrap())).await;
    }
    channel.send(ProgressEvent::Progress { chunk: 4, }).unwrap();
}

