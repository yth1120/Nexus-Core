use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::platform::{PlatformManager, SystemConnection};
use crate::utils::{AppError, AppResult};

pub struct MacOSPlatformManager;

impl PlatformManager for MacOSPlatformManager {
    fn set_system_proxy(&self, enabled: bool) {
        tracing::info!("[macOS] set_system_proxy({})", enabled);
    }

    fn get_system_proxy(&self) -> Option<String> {
        None
    }

    fn get_default_interface(&self) -> String {
        "en0".to_string()
    }

    fn get_network_interfaces(&self) -> Vec<String> {
        vec!["127.0.0.1".to_string()]
    }

    fn enable_auto_start(&self) -> AppResult<()> {
        // Create ~/Library/LaunchAgents/com.nexuscore.app.plist
        let launch_agents_dir = home_dir()?.join("Library").join("LaunchAgents");
        fs::create_dir_all(&launch_agents_dir)?;

        let exe_path = std::env::current_exe()
            .map_err(|e| AppError::Internal(format!("Cannot get exe path: {}", e)))?;

        let plist = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.nexuscore.app</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
            exe_path.display()
        );

        fs::write(launch_agents_dir.join("com.nexuscore.app.plist"), plist)?;
        tracing::info!("[macOS] Auto-start enabled");
        Ok(())
    }

    fn disable_auto_start(&self) -> AppResult<()> {
        let plist = home_dir()?
            .join("Library")
            .join("LaunchAgents")
            .join("com.nexuscore.app.plist");
        if plist.exists() {
            fs::remove_file(plist)?;
        }
        tracing::info!("[macOS] Auto-start disabled");
        Ok(())
    }

    fn enable_system_proxy(&self, host: &str, port: u16) -> AppResult<()> {
        let proxy_addr = format!("{}:{}", host, port);

        // Get the active network service name
        let service = get_active_network_service();

        // Set HTTP proxy
        Command::new("networksetup")
            .args(["-setwebproxy", &service, host, &port.to_string()])
            .output()?;

        // Set HTTPS proxy
        Command::new("networksetup")
            .args(["-setsecurewebproxy", &service, host, &port.to_string()])
            .output()?;

        tracing::info!(
            "[macOS] System proxy enabled: {} via {}",
            proxy_addr,
            service
        );
        Ok(())
    }

    fn disable_system_proxy(&self) -> AppResult<()> {
        let service = get_active_network_service();
        Command::new("networksetup")
            .args(["-setwebproxystate", &service, "off"])
            .output()?;
        Command::new("networksetup")
            .args(["-setsecurewebproxystate", &service, "off"])
            .output()?;

        tracing::info!("[macOS] System proxy disabled");
        Ok(())
    }

    fn open_logs_dir(&self) -> AppResult<()> {
        let logs_dir = directories::ProjectDirs::from("com", "NexusCore", "Nexus Core")
            .map(|d| d.data_dir().join("logs"))
            .ok_or_else(|| AppError::Internal("Cannot resolve logs dir".into()))?;
        Command::new("open")
            .arg(logs_dir.display().to_string())
            .spawn()?;
        Ok(())
    }

    fn open_config_dir(&self) -> AppResult<()> {
        let config_dir = directories::ProjectDirs::from("com", "NexusCore", "Nexus Core")
            .map(|d| d.data_dir().join("config"))
            .ok_or_else(|| AppError::Internal("Cannot resolve config dir".into()))?;
        Command::new("open")
            .arg(config_dir.display().to_string())
            .spawn()?;
        Ok(())
    }

    fn show_notification(&self, title: &str, body: &str) -> AppResult<()> {
        let script = format!(r#"display notification "{}" with title "{}""#, body, title);
        let _ = Command::new("osascript").args(["-e", &script]).output();
        Ok(())
    }

    fn get_active_connections(&self) -> AppResult<Vec<SystemConnection>> {
        // Use lsof to get active network connections
        let output = Command::new("lsof")
            .args(["-i", "-P", "-n", "-F", "pcRLn"])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut connections = Vec::new();
        let mut current_pid: u32 = 0;
        let mut current_process = String::new();
        let mut current_src = String::new();
        let mut current_dst = String::new();
        let mut current_proto = String::new();

        for line in stdout.lines() {
            match line.chars().next() {
                Some('p') => current_pid = line[1..].parse().unwrap_or(0),
                Some('c') => current_process = line[1..].to_string(),
                Some('R') => {
                    // Parse "TCP 127.0.0.1:54321->149.154.167.50:443"
                    let addr_part = &line[1..];
                    if let Some(arrow_pos) = addr_part.find("->") {
                        current_proto = if addr_part.starts_with("TCP") {
                            "TCP".into()
                        } else {
                            "UDP".into()
                        };
                        let proto_end = 3; // TCP/UDP prefix is always 3 chars
                        current_src = addr_part[proto_end..arrow_pos].trim().to_string();
                        current_dst = addr_part[arrow_pos + 2..].trim().to_string();
                    }
                }
                Some('L') => {
                    let state_str = &line[1..];
                    let state = if state_str.contains("ESTABLISHED") {
                        "ESTABLISHED"
                    } else if state_str.contains("LISTEN") {
                        "LISTEN"
                    } else {
                        continue;
                    };

                    if current_pid > 0 && state == "ESTABLISHED" {
                        connections.push(SystemConnection {
                            pid: current_pid,
                            process_name: current_process.clone(),
                            source: current_src.clone(),
                            destination: current_dst.clone(),
                            protocol: current_proto.clone(),
                            state: state.to_string(),
                            duration_secs: 0,
                            upload_bytes: 0,
                            download_bytes: 0,
                        });
                    }
                }
                _ => {}
            }
        }

        Ok(connections)
    }
}

/// Get the active network service name (e.g., "Wi-Fi", "Ethernet").
fn get_active_network_service() -> String {
    if let Ok(output) = Command::new("networksetup")
        .args(["-listallnetworkservices"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            let name = line.trim();
            if !name.is_empty() && !name.starts_with("An asterisk") {
                // Return the first non-header service
                return name.to_string();
            }
        }
    }
    "Wi-Fi".to_string()
}

fn home_dir() -> AppResult<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| AppError::Internal("Cannot resolve HOME directory".into()))
}
