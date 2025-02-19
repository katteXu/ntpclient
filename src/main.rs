#![windows_subsystem = "windows"]

use anyhow::Result;
use ntpclient::boot;
use ntpclient::utils::is_running_process;
#[cfg(unix)]
fn main() -> Result<()> {
    use fork::{daemon, Fork};

    is_running_process();

    // 判断进程是否存在
    match daemon(false, false) {
        Ok(Fork::Parent(_)) => {
            // Parent process exits
            std::process::exit(0);
        }
        Ok(Fork::Child) => {
            boot::start();
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
fn main() -> Result<()> {
    is_running_process();

    boot::start();

    Ok(())
}
