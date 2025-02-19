#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::fs::File;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::io::Write;

use std::env;
use std::path::PathBuf;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        // 获取当前可执行文件的路径
        let exe_path = env::current_exe()?;
        let exe_path_str = exe_path
            .to_str()
            .ok_or("Failed to convert path to string")?;

        // 打开注册表中的开机自启项所在的键
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let run_key = hkcu.open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            KEY_SET_VALUE,
        )?;

        // 获取程序的名称
        let program_name = exe_path
            .file_name()
            .ok_or("Failed to get program name")?
            .to_str()
            .ok_or("Failed to convert name to string")?;

        // 在注册表中设置开机自启项
        run_key.set_value(program_name, &exe_path_str)?;

        println!("已将程序设置为开机自动启动");
    }

    #[cfg(target_os = "linux")]
    {
        use auto_launch::*;
        let current_exe = std::env::current_exe().unwrap();
        let current_path = current_exe.to_str().unwrap();
        let app_name = "ntpclient.service";
        let app_path = current_path;
        let auto = AutoLaunch::new(app_name, app_path, &[] as &[&str]);

        auto.enable().unwrap();
        println!("已将程序设置为开机自动启动");
    }

    #[cfg(target_os = "macos")]
    {
        // 获取当前可执行文件的路径
        let exe_path = env::current_exe()?;
        let exe_path_str = exe_path
            .to_str()
            .ok_or("Failed to convert path to string")?;

        // 生成 plist 文件内容
        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF - 8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>ntp_service</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
            exe_path_str
        );

        // 定义 plist 文件的路径
        let mut launch_agents_dir = PathBuf::from(env::var("HOME")?);
        launch_agents_dir.push("Library/LaunchAgents");
        let plist_file_path = launch_agents_dir.join("ntp_service.plist");

        // 创建并写入 plist 文件
        let mut plist_file = File::create(plist_file_path)?;
        plist_file.write_all(plist_content.as_bytes())?;

        println!("已将程序设置为开机自动启动");
    }

    Ok(())
}
