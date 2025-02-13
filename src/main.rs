#![windows_subsystem = "windows"]

use anyhow::Result;
use ntpclient::boot;
#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<()> {
    use fork::{daemon, Fork};
    use ntpclient::utils::is_running_process;

    if is_running_process() {
        std::process::exit(0);
    }
    // 判断进程是否存在
    match daemon(false, false) {
        Ok(Fork::Parent(_)) => {
            // Parent process exits
            std::process::exit(0);
        }
        Ok(Fork::Child) => {
            boot::start().await;
        }
        Err(e) => {
            // Failed to daemonize
            eprintln!("Failed to daemonize: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
