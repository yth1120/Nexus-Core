use rand::Rng;

use crate::core::AppState;
use crate::models::Connection;
use crate::utils::AppResult;

/// Get all connections with fresh random traffic values on each call.
/// Does NOT mutate stored connections — returns copies with updated upload/download/duration.
pub fn get_all(state: &AppState) -> Vec<Connection> {
    let mut rng = rand::thread_rng();
    let connections = state.connections.read();

    if connections.is_empty() {
        // Generate some mock connections on first call
        drop(connections);
        let mock = generate_mock_connections(25);
        *state.connections.write() = mock.clone();
        return mock;
    }

    connections
        .iter()
        .map(|c| Connection {
            upload: if rng.gen::<f64>() > 0.6 {
                rng.gen_range(0..15 * 1024) as i64
            } else {
                0
            },
            download: rng.gen_range(0..800 * 1024) as i64,
            duration: c.duration + 1.0,
            ..c.clone()
        })
        .collect()
}

/// Close a connection by ID.
pub fn close_by_id(state: &AppState, id: &str) -> AppResult<()> {
    let mut connections = state.connections.write();
    let len_before = connections.len();
    connections.retain(|c| c.id != id);
    if connections.len() == len_before {
        return Err(crate::utils::AppError::NotFound(format!(
            "Connection {}",
            id
        )));
    }
    Ok(())
}

/// Close all connections.
pub fn close_all(state: &AppState) {
    state.connections.write().clear();
}

// ===== Mock Connection Generator =====

const PROCESSES: &[&str] = &[
    "Telegram.exe",
    "WeChat.exe",
    "svchost.exe",
    "Code.exe",
    "chrome.exe",
    "Discord.exe",
    "CoreSync.exe",
    "msedge.exe",
    "firefox.exe",
    "Slack.exe",
];

const DESTINATIONS: &[&str] = &[
    "gateway.icloud.com",
    "mtalk.google.com",
    "cdn.discordapp.com",
    "149.154.167.50:443",
    "api.twitter.com",
    "push.apple.com",
    "stun.l.google.com:19302",
    "api.github.com",
    "s3.amazonaws.com",
    "cloudflare-dns.com",
];

const RULE_NAMES: &[&str] = &[
    "Proxy (HK-01)",
    "Proxy (Auto)",
    "Direct (LAN)",
    "Reject (Ads)",
    "Direct (CN IP)",
    "Proxy (JP-01)",
    "Proxy (SG-01)",
    "DIRECT",
];

const NETWORKS: &[crate::models::NetworkProtocol] = &[
    crate::models::NetworkProtocol::TCP,
    crate::models::NetworkProtocol::UDP,
];

fn generate_mock_connections(count: usize) -> Vec<Connection> {
    let mut rng = rand::thread_rng();
    let now = chrono::Utc::now().timestamp_millis();
    let mut connections = Vec::with_capacity(count);

    for i in 0..count {
        connections.push(Connection {
            id: format!("conn-{}", i + 1),
            process: PROCESSES[rng.gen_range(0..PROCESSES.len())].to_string(),
            source: format!("127.0.0.1:{}", 50000 + rng.gen_range(0..10000u32)),
            destination: DESTINATIONS[rng.gen_range(0..DESTINATIONS.len())].to_string(),
            rule: RULE_NAMES[rng.gen_range(0..RULE_NAMES.len())].to_string(),
            network: match NETWORKS[rng.gen_range(0..NETWORKS.len())] {
                crate::models::NetworkProtocol::TCP => crate::models::NetworkProtocol::TCP,
                crate::models::NetworkProtocol::UDP => crate::models::NetworkProtocol::UDP,
            },
            upload: if rng.gen::<f64>() > 0.6 {
                rng.gen_range(0..15 * 1024) as i64
            } else {
                0
            },
            download: rng.gen_range(0..800 * 1024) as i64,
            duration: rng.gen_range(0.0..600.0),
            created_at: now - rng.gen_range(0..600000),
        });
    }

    connections
}
