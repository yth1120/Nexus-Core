use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub binary_path: String,
    pub config_path: String,
    pub args: Vec<String>,
    pub envs: HashMap<String, String>,
    pub work_dir: String,
    pub timeout_secs: u64,
}

impl ProcessConfig {
    pub fn new(binary: &str, config: &str) -> Self {
        Self {
            binary_path: binary.to_string(),
            config_path: config.to_string(),
            args: vec!["run".into(), "-c".into(), config.to_string()],
            envs: HashMap::new(),
            work_dir: ".".to_string(),
            timeout_secs: 30,
        }
    }
}
