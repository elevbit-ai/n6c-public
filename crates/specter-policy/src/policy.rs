use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub automatic_channel_change: bool,
    pub minimum_confidence: f64,
    pub cooldown_seconds: u64,
    pub max_changes_per_hour: u32,
    pub rollback_timeout_secs: u64,
    pub minimum_improvement_db: f64,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            automatic_channel_change: true,
            minimum_confidence: 0.85,
            cooldown_seconds: 120,
            max_changes_per_hour: 4,
            rollback_timeout_secs: 30,
            minimum_improvement_db: 3.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizedChannel {
    pub frequency_hz: u64,
    pub channel: u32,
    pub bandwidth_hz: u64,
    pub max_power_dbm: f64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelScore {
    pub channel: u32,
    pub frequency_hz: u64,
    pub interference_score: f64,
    pub occupancy_score: f64,
    pub noise_score: f64,
    pub instability_score: f64,
    pub switching_cost: f64,
    pub total_score: f64,
}

pub struct PolicyEngine {
    config: PolicyConfig,
    allowed_channels: Vec<AuthorizedChannel>,
    change_history: Vec<DateTime<Utc>>,
    protected_channels: Vec<u32>,
    forbidden_channels: Vec<u32>,
}

impl PolicyEngine {
    pub fn new(config: PolicyConfig) -> Self {
        Self {
            config,
            allowed_channels: Vec::new(),
            change_history: Vec::new(),
            protected_channels: Vec::new(),
            forbidden_channels: Vec::new(),
        }
    }

    pub fn set_allowed_channels(&mut self, channels: Vec<AuthorizedChannel>) {
        self.allowed_channels = channels;
    }

    pub fn set_protected_channels(&mut self, channels: Vec<u32>) {
        self.protected_channels = channels;
    }

    pub fn set_forbidden_channels(&mut self, channels: Vec<u32>) {
        self.forbidden_channels = channels;
    }

    pub fn can_change_channel(&self, current_channel: u32) -> bool {
        if !self.config.automatic_channel_change {
            return false;
        }
        if self.protected_channels.contains(&current_channel) {
            return false;
        }
        if self.is_in_cooldown() {
            return false;
        }
        if self.changes_last_hour() >= self.config.max_changes_per_hour {
            return false;
        }
        true
    }

    pub fn select_best_channel(
        &self,
        current_channel: u32,
        channel_scores: &[ChannelScore],
    ) -> Option<&ChannelScore> {
        let candidates: Vec<&ChannelScore> = channel_scores
            .iter()
            .filter(|s| {
                s.channel != current_channel
                    && !self.protected_channels.contains(&s.channel)
                    && !self.forbidden_channels.contains(&s.channel)
                    && self.allowed_channels.iter().any(|a| a.channel == s.channel)
            })
            .collect();
        candidates.into_iter().min_by(|a, b| {
            a.total_score
                .partial_cmp(&b.total_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn score_channels(
        &self,
        channel_data: &[(u32, u64, f64, f64, f64)],
        current_channel: u32,
    ) -> Vec<ChannelScore> {
        channel_data
            .iter()
            .map(|&(channel, freq, interference, occupancy, noise)| {
                let switching_cost = if channel == current_channel {
                    0.0
                } else {
                    0.1
                };
                let total = interference + occupancy + noise + switching_cost;
                ChannelScore {
                    channel,
                    frequency_hz: freq,
                    interference_score: interference,
                    occupancy_score: occupancy,
                    noise_score: noise,
                    instability_score: 0.0,
                    switching_cost,
                    total_score: total,
                }
            })
            .collect()
    }

    pub fn record_change(&mut self) {
        self.change_history.push(Utc::now());
        self.change_history
            .retain(|t| (Utc::now() - *t).num_seconds() < 3600);
    }

    pub fn is_in_cooldown(&self) -> bool {
        self.change_history.last().map_or(false, |last| {
            (Utc::now() - *last).num_seconds() < self.config.cooldown_seconds as i64
        })
    }

    pub fn changes_last_hour(&self) -> u32 {
        let cutoff = Utc::now() - chrono::Duration::hours(1);
        self.change_history
            .iter()
            .filter(|t| **t > cutoff)
            .count() as u32
    }

    pub fn config(&self) -> &PolicyConfig {
        &self.config
    }

    pub fn allowed_channels(&self) -> &[AuthorizedChannel] {
        &self.allowed_channels
    }

    pub fn validate_command(
        &self,
        radio_id: Uuid,
        target_channel: u32,
        confidence: f64,
    ) -> Result<(), String> {
        if !self.config.automatic_channel_change {
            return Err("Automatic channel change is disabled".to_string());
        }
        if confidence < self.config.minimum_confidence {
            return Err(format!(
                "Confidence {} below minimum {}",
                confidence, self.config.minimum_confidence
            ));
        }
        if self.forbidden_channels.contains(&target_channel) {
            return Err(format!("Channel {} is forbidden", target_channel));
        }
        if self.protected_channels.contains(&target_channel) {
            return Err(format!("Channel {} is protected", target_channel));
        }
        if !self.allowed_channels.iter().any(|a| a.channel == target_channel) {
            return Err(format!("Channel {} is not in allowlist", target_channel));
        }
        if self.is_in_cooldown() {
            return Err("Cooldown period active".to_string());
        }
        if self.changes_last_hour() >= self.config.max_changes_per_hour {
            return Err("Maximum changes per hour exceeded".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_cooldown() {
        let mut engine = PolicyEngine::new(PolicyConfig {
            cooldown_seconds: 60,
            max_changes_per_hour: 4,
            ..Default::default()
        });
        assert!(!engine.is_in_cooldown());
        engine.record_change();
        assert!(engine.is_in_cooldown());
    }

    #[test]
    fn test_policy_allowlist() {
        let mut engine = PolicyEngine::new(PolicyConfig::default());
        engine.set_allowed_channels(vec![AuthorizedChannel {
            frequency_hz: 433_100_000,
            channel: 1,
            bandwidth_hz: 25_000,
            max_power_dbm: 20.0,
            label: "Ch1".to_string(),
        }]);
        assert!(engine.allowed_channels().len() == 1);
    }
}
