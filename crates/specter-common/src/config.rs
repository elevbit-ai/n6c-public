use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub system: SystemSection,
    pub rf: RFSection,
    pub policy: PolicySection,
    pub security: SecuritySection,
    pub database: DatabaseSection,
    pub server: ServerSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSection {
    pub site_name: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFSection {
    pub device: String,
    pub sample_rate: u32,
    pub center_frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub gain: f64,
    pub fft_size: u32,
    pub window_function: String,
    pub overlap: f64,
    pub dwell_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySection {
    pub automatic_channel_change: bool,
    pub minimum_confidence: f64,
    pub cooldown_seconds: u64,
    pub max_changes_per_hour: u32,
    pub rollback_timeout_secs: u64,
    pub minimum_improvement_db: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySection {
    pub require_mtls: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    pub ca_cert_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSection {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSection {
    pub bind_address: String,
    pub port: u16,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            system: SystemSection {
                site_name: "LAB-01".to_string(),
                log_level: "info".to_string(),
            },
            rf: RFSection {
                device: "soapy".to_string(),
                sample_rate: 2_400_000,
                center_frequency_hz: 433_000_000,
                bandwidth_hz: 2_000_000,
                gain: 30.0,
                fft_size: 4096,
                window_function: "hann".to_string(),
                overlap: 0.5,
                dwell_time_ms: 100,
            },
            policy: PolicySection {
                automatic_channel_change: true,
                minimum_confidence: 0.85,
                cooldown_seconds: 120,
                max_changes_per_hour: 4,
                rollback_timeout_secs: 30,
                minimum_improvement_db: 3.0,
            },
            security: SecuritySection {
                require_mtls: true,
                tls_cert_path: None,
                tls_key_path: None,
                ca_cert_path: None,
            },
            database: DatabaseSection {
                url: "postgres://specter:specter@localhost/specter_net".to_string(),
                max_connections: 10,
            },
            server: ServerSection {
                bind_address: "0.0.0.0".to_string(),
                port: 8080,
            },
        }
    }
}

impl SystemConfig {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
