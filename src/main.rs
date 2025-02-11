#![windows_subsystem = "windows"]

use anyhow::Result;
use ntpclient::boot;
use single_instance::SingleInstance;
#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<()> {
    use fork::{daemon, Fork};
    let instance = SingleInstance::new("ntp-client")?;
    if instance.is_single() {
        if let Ok(Fork::Child) = daemon(false, false) {
            boot::start().await;
        }
    }

    Ok(())
}

#[cfg(windows)]
#[tokio::main]
async fn main() -> Result<()> {
    let instance = SingleInstance::new("ntp-client")?;
    if instance.is_single() {
        boot::start().await;
    }

    Ok(())
}
