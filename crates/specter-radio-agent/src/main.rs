use anyhow::Result;
use specter_radio_agent::{RadioAgent, NullRadioController};
use specter_policy::{PolicyEngine, PolicyConfig, AuthorizedChannel};
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
    tracing::info!("SPECTER-NET Radio Agent starting...");
    let policy = PolicyEngine::new(PolicyConfig {
        automatic_channel_change: config.policy.automatic_channel_change,
        minimum_confidence: config.policy.minimum_confidence,
        cooldown_seconds: config.policy.cooldown_seconds,
        max_changes_per_hour: config.policy.max_changes_per_hour,
        rollback_timeout_secs: config.policy.rollback_timeout_secs,
        minimum_improvement_db: config.policy.minimum_improvement_db,
    });
    let controller = NullRadioController::new(
        uuid::Uuid::new_v4(),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    let mut agent = RadioAgent::new(
        uuid::Uuid::new_v4(),
        Box::new(controller),
        policy,
    );
    agent.monitor().await
}
