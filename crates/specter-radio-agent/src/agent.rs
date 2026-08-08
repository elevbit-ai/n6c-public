use anyhow::Result;
use specter_common::*;
use specter_policy::PolicyEngine;
use std::time::Duration;
use tracing::{error, info, warn};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait RadioController: Send + Sync {
    async fn get_status(&self) -> Result<RadioStatus>;
    async fn get_current_channel(&self) -> Result<u32>;
    async fn list_allowed_channels(&self) -> Result<Vec<u32>>;
    async fn set_channel(&self, channel: u32, frequency_hz: u64) -> Result<()>;
    async fn verify_connectivity(&self) -> Result<bool>;
    async fn rollback(&self, channel: u32, frequency_hz: u64) -> Result<()>;
}

pub struct NullRadioController {
    radio_id: Uuid,
    current_channel: u32,
    allowed_channels: Vec<u32>,
}

impl NullRadioController {
    pub fn new(radio_id: Uuid, allowed_channels: Vec<u32>) -> Self {
        Self {
            radio_id,
            current_channel: allowed_channels.first().copied().unwrap_or(1),
            allowed_channels,
        }
    }
}

#[async_trait::async_trait]
impl RadioController for NullRadioController {
    async fn get_status(&self) -> Result<RadioStatus> {
        Ok(RadioStatus::Online)
    }

    async fn get_current_channel(&self) -> Result<u32> {
        Ok(self.current_channel)
    }

    async fn list_allowed_channels(&self) -> Result<Vec<u32>> {
        Ok(self.allowed_channels.clone())
    }

    async fn set_channel(&self, _channel: u32, _frequency_hz: u64) -> Result<()> {
        warn!("NullRadioController: set_channel called (no real radio)");
        Ok(())
    }

    async fn verify_connectivity(&self) -> Result<bool> {
        Ok(true)
    }

    async fn rollback(&self, _channel: u32, _frequency_hz: u64) -> Result<()> {
        warn!("NullRadioController: rollback called (no real radio)");
        Ok(())
    }
}

pub struct RadioAgent {
    radio_id: Uuid,
    controller: Box<dyn RadioController>,
    policy: PolicyEngine,
    state: RadioStatus,
    change_state: ChannelChangeState,
}

impl RadioAgent {
    pub fn new(
        radio_id: Uuid,
        controller: Box<dyn RadioController>,
        policy: PolicyEngine,
    ) -> Self {
        Self {
            radio_id,
            controller,
            policy,
            state: RadioStatus::Online,
            change_state: ChannelChangeState::Stable,
        }
    }

    pub async fn handle_channel_change(
        &mut self,
        target_channel: u32,
        target_frequency_hz: u64,
        confidence: f64,
    ) -> Result<ChannelChange> {
        let current_channel = self.controller.get_current_channel().await?;
        info!(
            "Processing channel change: radio={}, from={}, to={}, confidence={}",
            self.radio_id, current_channel, target_channel, confidence
        );
        if let Err(e) = self.policy.validate_command(self.radio_id, target_channel, confidence) {
            error!("Policy validation failed: {}", e);
            return Err(anyhow::anyhow!("Policy violation: {}", e));
        }
        self.change_state = ChannelChangeState::CandidateSelected;
        self.change_state = ChannelChangeState::ChangePending;
        self.change_state = ChannelChangeState::Changing;
        match self.controller.set_channel(target_channel, target_frequency_hz).await {
            Ok(()) => {
                self.change_state = ChannelChangeState::Verifying;
                tokio::time::sleep(Duration::from_secs(2)).await;
                if self.controller.verify_connectivity().await? {
                    self.change_state = ChannelChangeState::StableNewChannel;
                    self.policy.record_change();
                    info!("Channel change successful: radio={}, new_channel={}", self.radio_id, target_channel);
                    Ok(ChannelChange {
                        id: Uuid::new_v4(),
                        radio_id: self.radio_id,
                        sensor_id: Uuid::new_v4(),
                        timestamp: chrono::Utc::now(),
                        from_channel: current_channel,
                        to_channel: target_channel,
                        from_frequency_hz: 0,
                        to_frequency_hz: target_frequency_hz,
                        reason: "Automatic channel change due to interference".to_string(),
                        state: ChannelChangeState::StableNewChannel,
                        success: Some(true),
                        rolled_back: false,
                    })
                } else {
                    warn!("Post-change verification failed, rolling back");
                    self.change_state = ChannelChangeState::Rollback;
                    let old_freq = self.controller.get_current_channel().await?;
                    self.controller.rollback(current_channel, 0).await?;
                    self.change_state = ChannelChangeState::Stable;
                    Ok(ChannelChange {
                        id: Uuid::new_v4(),
                        radio_id: self.radio_id,
                        sensor_id: Uuid::new_v4(),
                        timestamp: chrono::Utc::now(),
                        from_channel: current_channel,
                        to_channel: target_channel,
                        from_frequency_hz: 0,
                        to_frequency_hz: target_frequency_hz,
                        reason: "Rolled back: verification failed".to_string(),
                        state: ChannelChangeState::Rollback,
                        success: Some(false),
                        rolled_back: true,
                    })
                }
            }
            Err(e) => {
                error!("Channel change failed: {}", e);
                self.change_state = ChannelChangeState::Rollback;
                self.controller.rollback(current_channel, 0).await?;
                self.change_state = ChannelChangeState::Stable;
                Ok(ChannelChange {
                    id: Uuid::new_v4(),
                    radio_id: self.radio_id,
                    sensor_id: Uuid::new_v4(),
                    timestamp: chrono::Utc::now(),
                    from_channel: current_channel,
                    to_channel: target_channel,
                    from_frequency_hz: 0,
                    to_frequency_hz: target_frequency_hz,
                    reason: format!("Failed: {}", e),
                    state: ChannelChangeState::Rollback,
                    success: Some(false),
                    rolled_back: true,
                })
            }
        }
    }

    pub async fn monitor(&mut self) -> Result<()> {
        info!("Radio agent monitoring: radio={}", self.radio_id);
        loop {
            match self.controller.verify_connectivity().await {
                Ok(true) => {
                    if self.state != RadioStatus::Online {
                        self.state = RadioStatus::Online;
                        info!("Radio {} back online", self.radio_id);
                    }
                }
                Ok(false) => {
                    if self.state == RadioStatus::Online {
                        self.state = RadioStatus::Offline;
                        warn!("Radio {} went offline", self.radio_id);
                    }
                }
                Err(e) => {
                    self.state = RadioStatus::Error;
                    error!("Radio {} error: {}", self.radio_id, e);
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    pub fn state(&self) -> &RadioStatus {
        &self.state
    }

    pub fn change_state(&self) -> &ChannelChangeState {
        &self.change_state
    }

    pub fn radio_id(&self) -> Uuid {
        self.radio_id
    }
}
