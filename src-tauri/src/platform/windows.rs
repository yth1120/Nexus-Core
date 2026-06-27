use std::process::Command;

use crate::platform::{PlatformManager, SystemConnection};
use crate::utils::{AppError, AppResult};

pub struct WindowsPlatformManager;

impl PlatformManager for WindowsPlatformManager {
    // ===== Legacy Phase 1 stubs =====

    fn set_system_proxy(&self, enabled: bool) {
        tracing::info!("[Windows] set_system_proxy({})", enabled);
    }

    fn get_system_proxy(&self) -> Option<String> {
        None
    }

    fn get_default_interface(&self) -> String {
        "Ethernet".to_string()
    }

    fn get_network_interfaces(&self) -> Vec<String> {
        vec!["127.0.0.1".to_string()]
    }

    // ===== Phase 2: Real implementations =====

    fn enable_auto_start(&self) -> AppResult<()> {
        // Add registry key: HKCU\Software\Microsoft\Windows\CurrentVersion\Run\NexusCore
        let exe_path = std::env::current_exe()
            .map_err(|e| AppError::Internal(format!("Cannot get exe path: {}", e)))?;
        let exe_str = exe_path.display().to_string();

        let output = Command::new("reg")
            .args([
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                "/v",
                "NexusCore",
                "/t",
                "REG_SZ",
                "/d",
                &exe_str,
                "/f",
            ])
            .output()?;

        if output.status.success() {
            tracing::info!("[Windows] Auto-start enabled");
            Ok(())
        } else {
            Err(AppError::Internal("Failed to set registry key".into()))
        }
    }

    fn disable_auto_start(&self) -> AppResult<()> {
        let output = Command::new("reg")
            .args([
                "delete",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                "/v",
                "NexusCore",
                "/f",
            ])
            .output()?;

        if output.status.success() {
            tracing::info!("[Windows] Auto-start disabled");
            Ok(())
        } else {
            // Key might not exist — that's fine
            tracing::info!("[Windows] Auto-start key not present (already disabled)");
            Ok(())
        }
    }

    fn enable_system_proxy(&self, host: &str, port: u16) -> AppResult<()> {
        let proxy_addr = format!("{}:{}", host, port);

        // Set proxy via registry (Internet Settings)
        Command::new("reg")
            .args([
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
                "/v",
                "ProxyEnable",
                "/t",
                "REG_DWORD",
                "/d",
                "1",
                "/f",
            ])
            .output()?;

        Command::new("reg")
            .args([
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
                "/v",
                "ProxyServer",
                "/t",
                "REG_SZ",
                "/d",
                &proxy_addr,
                "/f",
            ])
            .output()?;

        tracing::info!("[Windows] System proxy enabled: {}", proxy_addr);
        Ok(())
    }

    fn disable_system_proxy(&self) -> AppResult<()> {
        Command::new("reg")
            .args([
                "add",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
                "/v",
                "ProxyEnable",
                "/t",
                "REG_DWORD",
                "/d",
                "0",
                "/f",
            ])
            .output()?;

        tracing::info!("[Windows] System proxy disabled");
        Ok(())
    }

    fn open_logs_dir(&self) -> AppResult<()> {
        let logs_dir = directories::ProjectDirs::from("com", "NexusCore", "Nexus Core")
            .map(|d| d.data_dir().join("logs"))
            .ok_or_else(|| AppError::Internal("Cannot resolve logs dir".into()))?;

        Command::new("explorer")
            .arg(logs_dir.display().to_string())
            .spawn()?;

        Ok(())
    }

    fn open_config_dir(&self) -> AppResult<()> {
        let config_dir = directories::ProjectDirs::from("com", "NexusCore", "Nexus Core")
            .map(|d| d.data_dir().join("config"))
            .ok_or_else(|| AppError::Internal("Cannot resolve config dir".into()))?;

        Command::new("explorer")
            .arg(config_dir.display().to_string())
            .spawn()?;

        Ok(())
    }

    fn show_notification(&self, title: &str, body: &str) -> AppResult<()> {
        // Use PowerShell to show a toast notification (simpler than WinRT COM)
        let ps_script = format!(
            r#"[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
            Add-Type -AssemblyName System.Windows.Forms
            $balloon = New-Object System.Windows.Forms.NotifyIcon
            $balloon.Icon = [System.Drawing.SystemIcons]::Information
            $balloon.BalloonTipTitle = '{}'
            $balloon.BalloonTipText = '{}'
            $balloon.Visible = $true
            $balloon.ShowBalloonTip(5000)
            Start-Sleep -Seconds 6
            $balloon.Dispose()"#,
            title, body
        );

        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_script])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            // Notifications might fail in non-interactive sessions; not critical
            tracing::debug!("[Windows] Notification failed (non-critical)");
            Ok(())
        }
    }

    fn get_active_connections(&self) -> AppResult<Vec<SystemConnection>> {
        // Use netstat to get active TCP/UDP connections
        let output = Command::new("netstat")
            .args(["-ano", "-p", "TCP"])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut connections = Vec::new();

        for line in stdout.lines().skip(4) {
            // Parse: "  TCP    127.0.0.1:54321    149.154.167.50:443    ESTABLISHED     1234"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 && parts[0] == "TCP" {
                let pid: u32 = parts.last().and_then(|s| s.parse().ok()).unwrap_or(0);
                let state = parts[parts.len() - 2].to_string();

                if state == "ESTABLISHED" && pid > 0 {
                    connections.push(SystemConnection {
                        pid,
                        process_name: resolve_process_name(pid),
                        source: parts.get(1).unwrap_or(&"unknown").to_string(),
                        destination: parts.get(2).unwrap_or(&"unknown").to_string(),
                        protocol: "TCP".into(),
                        state,
                        duration_secs: 0,
                        upload_bytes: 0,
                        download_bytes: 0,
                    });
                }
            }
        }

        Ok(connections)
    }
}

/// Resolve a PID to a process name using tasklist.
fn resolve_process_name(pid: u32) -> String {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/NH"])
        .output()
        .ok();

    if let Some(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // CSV format: "process.exe","1234","Console","1","12,345 K"
        for line in stdout.lines() {
            if let Some(name) = line.split(',').next() {
                let clean = name.trim_matches('"').to_string();
                if !clean.is_empty() {
                    return clean;
                }
            }
        }
    }

    format!("pid_{}", pid)
}
