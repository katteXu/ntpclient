use anyhow::{anyhow, Result};

use base64::Engine;
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};

#[cfg(target_os = "macos")]
use std::process::Command;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    net::TcpListener,
    process, thread,
};

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

    let client_id = format!("{}-{}", cpu_id, mac_address);
    // client_id 做hash 算法然后获取 前10位
    let client_id = hash_string_and_get_first_10(&client_id);

    let result = format!("{}-|-{}-|-{}", client_id, whoami, hostname);

    let result = base64::engine::general_purpose::STANDARD.encode(result);
    Ok(result)
}

pub fn is_running_process() {
    let listner = TcpListener::bind("127.0.0.1:12340");
    let listner = match listner {
        Ok(listner) => listner,
        Err(_) => {
            println!("程序已存在, 请勿重复运行");
            process::exit(0);
        }
    };
    thread::spawn(move || for _stream in listner.incoming() {});
}

fn hash_string_and_get_first_10(s: &str) -> String {
    // 创建一个 DefaultHasher 实例
    let mut hasher = DefaultHasher::new();
    // 对输入的字符串进行哈希计算
    s.hash(&mut hasher);
    // 获取哈希结果
    let hash_result = hasher.finish();
    // 将哈希结果转换为十六进制字符串
    let hex_string = format!("{:x}", hash_result);
    // 获取前 10 位，如果结果长度不足 10 位则取整个字符串
    let first_10_chars = &hex_string[0..10.min(hex_string.len())];
    first_10_chars.to_string()
}

#[test]
fn test_get_client_id() {
    let client_id = get_client_id().unwrap();
    println!("client_id:{}", client_id);
}
