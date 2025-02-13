use anyhow::{anyhow, Result};

use base64::Engine;
use sysinfo::{ProcessesToUpdate, System};
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};

#[cfg(target_os = "macos")]
use std::process::Command;

// 获取 CPU ID
fn get_cpu_id() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe {
            GetSystemInfo(&mut sys_info);
        }
        let processor_type = sys_info.dwProcessorType;
        Some(format!("{:x}", processor_type))
    }
    #[cfg(target_os = "linux")]
    {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let cpuinfo_path = "/proc/cpuinfo";
        if let Ok(file) = File::open(cpuinfo_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("processor") {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() == 2 {
                            let cpu_id = parts[1].trim();
                            return Some(cpu_id.to_string());
                        }
                    }
                }
            }
        }
        None
    }
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
            .ok()?;
        if output.status.success() {
            let cpu_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Some(cpu_id)
        } else {
            None
        }
    }
}

// 获取 MAC 地址
fn get_mac_address() -> Option<String> {
    let mac_result = mac_address::get_mac_address().unwrap();

    mac_result.map(|mac| mac.to_string())
}

pub fn get_client_id() -> Result<String> {
    let cpu_id = get_cpu_id().ok_or(anyhow!("Failed to get CPU ID"))?;
    let mac_address = get_mac_address().ok_or(anyhow!("Failed to get MAC address"))?;
    let whoami = whoami::username();
    let hostname = whoami::fallible::hostname()?;

    let client_id = format!("{}-{}-{}-{}", cpu_id, mac_address, whoami, hostname);
    println!("cpu id:{}", cpu_id);
    println!("mac address:{}", mac_address);
    println!("whoami:{}", whoami);
    println!("hostname:{}", hostname);
    let result = base64::engine::general_purpose::STANDARD.encode(client_id);
    // 截取前 16 位
    let result = result.get(..10).unwrap_or(&result).to_string();
    Ok(result)
}

pub fn is_running_process() -> bool {
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::All, true);

    // 获取当前进程信息
    let current_pid = sysinfo::get_current_pid().unwrap();
    let current_process = system.process(current_pid).unwrap();
    let current_exe = current_process.exe();
    let current_name = current_exe
        .unwrap()
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .trim_end_matches(".exe"); // 去除 .exe 后缀

    // 遍历进程列表
    for process in system.processes().values() {
        if process.pid() != current_pid {
            if let Some(exe_path) = process.exe() {
                let target_name = exe_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .trim_end_matches(".exe");

                if target_name == current_name {
                    return true;
                }
            }
        }
    }

    false
}

#[test]
fn test_get_client_id() {
    let client_id = get_client_id().unwrap();
    println!("client_id:{}", client_id);
}
