#![windows_subsystem = "windows"]

use anyhow::Result;
use ntpclient::boot;
use ntpclient::utils::is_running_process;
#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<()> {
    use fork::{daemon, Fork};

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

#[cfg(windows)]
#[tokio::main]
async fn main() -> Result<()> {
    if is_running_process() {
        std::process::exit(0);
    }

    boot::start().await;

    Ok(())
}
