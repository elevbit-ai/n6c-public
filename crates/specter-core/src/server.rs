use crate::api::{AppState, SharedState, create_router};
use anyhow::Result;
use specter_common::SystemConfig;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct SpecterCore {
    config: SystemConfig,
    state: SharedState,
}

impl SpecterCore {
    pub fn new(config: SystemConfig) -> Self {
        let state = Arc::new(RwLock::new(AppState::default()));
        Self { config, state }
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!(
            "{}:{}",
            self.config.server.bind_address, self.config.server.port
        );
        info!("Starting SPECTER-NET Core on {}", addr);
        let router = create_router(self.state.clone());
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, router).await?;
        Ok(())
    }

    pub fn state(&self) -> SharedState {
        self.state.clone()
    }
}
