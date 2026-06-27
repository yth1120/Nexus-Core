use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::platform::{PlatformManager, SystemConnection};
use crate::utils::{AppError, AppResult};

pub struct LinuxPlatformManager;

impl PlatformManager for LinuxPlatformManager {
    fn set_system_proxy(&self, enabled: bool) {
        tracing::info!("[Linux] set_system_proxy({})", enabled);
    }

    fn get_system_proxy(&self) -> Option<String> {
        std::env::var("http_proxy").ok()
    }

    fn get_default_interface(&self) -> String {
        "eth0".to_string()
    }

    fn get_network_interfaces(&self) -> Vec<String> {
        vec!["127.0.0.1".to_string()]
    }

    fn enable_auto_start(&self) -> AppResult<()> {
        // Create ~/.config/autostart/nexus-core.desktop
        let autostart_dir = dirs_autostart()?;
        fs::create_dir_all(&autostart_dir)?;

        let exe_path = std::env::current_exe()
            .map_err(|e| AppError::Internal(format!("Cannot get exe path: {}", e)))?;

        let desktop_entry = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=Nexus Core\n\
             Exec={}\n\
             X-GNOME-Autostart-enabled=true\n\
             Terminal=false\n\
             NoDisplay=false\n",
            exe_path.display()
        );

        fs::write(autostart_dir.join("nexus-core.desktop"), desktop_entry)?;
        tracing::info!("[Linux] Auto-start enabled");
        Ok(())
    }

    fn disable_auto_start(&self) -> AppResult<()> {
        let autostart_dir = dirs_autostart()?;
        let desktop_file = autostart_dir.join("nexus-core.desktop");
        if desktop_file.exists() {
            fs::remove_file(desktop_file)?;
        }
        tracing::info!("[Linux] Auto-start disabled");
        Ok(())
    }

    fn enable_system_proxy(&self, host: &str, port: u16) -> AppResult<()> {
        // Try gsettings (GNOME), fallback to environment variables
        let proxy_addr = format!("{}:{}", host, port);
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", "manual"])
            .output();
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.http", "host", host])
            .output();
        let _ = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.system.proxy.http",
                "port",
                &port.to_string(),
            ])
            .output();

        // Also set environment variables for terminal apps
        std::env::set_var("http_proxy", &proxy_addr);
        std::env::set_var("https_proxy", &proxy_addr);

        tracing::info!("[Linux] System proxy enabled: {}", proxy_addr);
        Ok(())
    }

    fn disable_system_proxy(&self) -> AppResult<()> {
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", "none"])
            .output();
        tracing::info!("[Linux] System proxy disabled");
        Ok(())
    }

    fn open_logs_dir(&self) -> AppResult<()> {
        let logs_dir = directories::ProjectDirs::from("com", "NexusCore", "Nexus Core")
            .map(|d| d.data_dir().join("logs"))
            .ok_or_else(|| AppError::Internal("Cannot resolve logs dir".into()))?;
        Command::new("xdg-open")
            .arg(logs_dir.display().to_string())
            .spawn()?;
        Ok(())
    }

    fn open_config_dir(&self) -> AppResult<()> {
        let config_dir = directories::ProjectDirs::from("com", "NexusCore", "Nexus Core")
            .map(|d| d.data_dir().join("config"))
            .ok_or_else(|| AppError::Internal("Cannot resolve config dir".into()))?;
        Command::new("xdg-open")
            .arg(config_dir.display().to_string())
            .spawn()?;
        Ok(())
    }

    fn show_notification(&self, title: &str, body: &str) -> AppResult<()> {
        let _ = Command::new("notify-send").args([title, body]).output();
        // notify-send might not be installed; not critical
        Ok(())
    }

    fn get_active_connections(&self) -> AppResult<Vec<SystemConnection>> {
        // Parse /proc/net/tcp (and /proc/net/tcp6, /proc/net/udp)
        let mut connections = Vec::new();

        // Parse TCP connections
        if let Ok(entries) = parse_proc_net_tcp("tcp") {
            connections.extend(entries);
        }
        if let Ok(entries) = parse_proc_net_tcp("tcp6") {
            connections.extend(entries);
        }
        if let Ok(entries) = parse_proc_net_tcp("udp") {
            connections.extend(entries);
        }

        Ok(connections)
    }
}

/// Parse /proc/net/{proto} to extract active connections.
fn parse_proc_net_tcp(proto: &str) -> AppResult<Vec<SystemConnection>> {
    let content = fs::read_to_string(format!("/proc/net/{}", proto))?;
    let mut connections = Vec::new();

    for line in content.lines().skip(1) {
        // Format: sl local_address rem_address st tx_queue rx_queue tr tm->when retrnsmt uid timeout inode
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 {
            continue;
        }

        let state_hex = parts[3];
        let uid: u32 = parts[7].parse().unwrap_or(0);

        // Only include ESTABLISHED connections (state 01 or 0A)
        let state = match state_hex {
            "01" => "ESTABLISHED",
            "0A" => "LISTEN",
            "06" => "TIME_WAIT",
            _ => continue,
        };

        if state != "ESTABLISHED" {
            continue;
        }

        let local = parse_hex_addr(parts[1]);
        let remote = parse_hex_addr(parts[2]);

        connections.push(SystemConnection {
            pid: 0, // PID not directly available in /proc/net/tcp; would need /proc/net/tcp + /proc/[pid]/fd
            process_name: resolve_linux_process(uid),
            source: local,
            destination: remote,
            protocol: if proto.contains("udp") {
                "UDP".into()
            } else {
                "TCP".into()
            },
            state: state.to_string(),
            duration_secs: 0,
            upload_bytes: 0,
            download_bytes: 0,
        });
    }

    Ok(connections)
}

/// Parse hex IP:port format from /proc/net (e.g., "00000000:1F90" → "0.0.0.0:8080")
fn parse_hex_addr(hex: &str) -> String {
    let parts: Vec<&str> = hex.split(':').collect();
    if parts.len() != 2 {
        return hex.to_string();
    }
    let ip_hex = u32::from_str_radix(parts[0], 16).unwrap_or(0);
    let port = u16::from_str_radix(parts[1], 16).unwrap_or(0);

    format!(
        "{}.{}.{}.{}:{}",
        ip_hex & 0xFF,
        (ip_hex >> 8) & 0xFF,
        (ip_hex >> 16) & 0xFF,
        (ip_hex >> 24) & 0xFF,
        port
    )
}

/// Resolve a UID to a username or process hint.
fn resolve_linux_process(_uid: u32) -> String {
    // Simplification: querying per-process FDs is expensive.
    // For a real implementation, iterate /proc/[pid]/fd/ looking for socket inodes.
    "linux-process".into()
}

fn dirs_autostart() -> AppResult<PathBuf> {
    let home = dirs_fallback::home_dir()
        .ok_or_else(|| AppError::Internal("Cannot resolve home directory".into()))?;
    Ok(home.join(".config").join("autostart"))
}

/// Minimal home_dir lookup without the `dirs` crate dependency.
mod dirs_fallback {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}
