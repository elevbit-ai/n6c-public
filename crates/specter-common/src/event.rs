use crate::types::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TelemetryMessage {
    pub schema_version: u32,
    pub sensor_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub sequence: u64,
    pub payload: TelemetryPayload,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TelemetryPayload {
    Spectrum(SpectrumMeasurement),
    Event(RFEvent),
    Status(SensorStatus),
    Heartbeat,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandMessage {
    pub command_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub nonce: String,
    pub payload: CommandPayload,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CommandPayload {
    ChangeChannel {
        radio_id: Uuid,
        target_channel: u32,
        target_frequency_hz: u64,
    },
    GetRadioStatus {
        radio_id: Uuid,
    },
    Ping,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandAck {
    pub command_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub message: String,
}
