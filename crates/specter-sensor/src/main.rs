use anyhow::Result;
use specter_sensor::SpecterSensor;
use specter_common::SystemConfig;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let config_path = std::env::var("SPECTER_CONFIG")
        .unwrap_or_else(|_| "/etc/specter-net/specter.toml".to_string());
    let config = SystemConfig::load(&config_path)
        .unwrap_or_else(|e| {
            tracing::warn!("Failed to load config from {}: {}", config_path, e);
            tracing::info!("Using default configuration");
            SystemConfig::default()
        });
    tracing::info!("SPECTER-NET Sensor starting...");
    tracing::info!("Device: {}", config.rf.device);
    tracing::info!("Sample rate: {} Hz", config.rf.sample_rate);
    tracing::info!("Center frequency: {} Hz", config.rf.center_frequency_hz);
    let mut sensor = SpecterSensor::new(config);
    sensor.initialize().await?;
    sensor.run_continuous().await?;
    Ok(())
}
