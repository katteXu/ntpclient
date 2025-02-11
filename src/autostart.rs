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
        // 获取当前可执行文件的路径
        let exe_path = env::current_exe()?;
        let exe_path_str = exe_path
            .to_str()
            .ok_or("Failed to convert path to string")?;

        // 定义 Systemd 服务单元文件内容
        let service_content = format!(
            "[Unit]\nDescription=My Rust Program\nAfter=network.target\n\n[Service]\nExecStart={}\nRestart=always\nUser=your_username\nGroup=your_groupname\n\n[Install]\nWantedBy=multi - user.target",
            exe_path_str
        );

        // 定义服务单元文件的路径
        let service_file_path = "/etc/systemd/system/my_rust_program.service";
        let mut service_file = File::create(service_file_path)?;
        service_file.write_all(service_content.as_bytes())?;

        // 重新加载 Systemd 管理器配置
        Command::new("systemctl").arg("daemon-reload").status()?;

        // 启用服务
        Command::new("systemctl")
            .arg("enable")
            .arg("my_rust_program.service")
            .status()?;

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
    <string>my_rust_program</string>
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
        let plist_file_path = launch_agents_dir.join("my_rust_program.plist");

        // 创建并写入 plist 文件
        let mut plist_file = File::create(plist_file_path)?;
        plist_file.write_all(plist_content.as_bytes())?;

        println!("已将程序设置为开机自动启动");
    }

    Ok(())
}
