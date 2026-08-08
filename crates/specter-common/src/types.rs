use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SensorStatus {
    Online,
    Offline,
    Degraded,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RadioStatus {
    Online,
    Offline,
    Degraded,
    ChangingChannel,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RFClassification {
    Normal,
    ElevatedNoise,
    InterferenceSuspected,
    JammingSuspected,
    Degraded,
    Recovering,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChannelChangeState {
    Stable,
    Degrading,
    CandidateSelected,
    ChangePending,
    Changing,
    Verifying,
    StableNewChannel,
    Rollback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorInfo {
    pub id: Uuid,
    pub name: String,
    pub location: String,
    pub status: SensorStatus,
    pub last_seen: DateTime<Utc>,
    pub center_frequency_hz: u64,
    pub sample_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioInfo {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub status: RadioStatus,
    pub current_channel: u32,
    pub current_frequency_hz: u64,
    pub allowed_channels: Vec<u32>,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectrumMeasurement {
    pub sensor_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub center_frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub sample_rate: u32,
    pub fft_size: u32,
    pub noise_floor_dbm: f64,
    pub peak_dbm: f64,
    pub mean_dbm: f64,
    pub snr_db: f64,
    pub occupancy: f64,
    pub psd: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelOccupancy {
    pub channel: u32,
    pub frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub occupancy: f64,
    pub mean_power_dbm: f64,
    pub peak_power_dbm: f64,
    pub noise_floor_dbm: f64,
    pub snr_db: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub schema_version: u32,
    pub sensor_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub sequence: u64,
    pub center_frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub noise_floor_dbm: f64,
    pub peak_dbm: f64,
    pub mean_dbm: f64,
    pub snr_db: f64,
    pub occupancy: f64,
    pub classification: RFClassification,
    pub confidence: f64,
    pub channels: Vec<ChannelOccupancy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFEvent {
    pub id: Uuid,
    pub sensor_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: RFClassification,
    pub confidence: f64,
    pub center_frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub duration_secs: f64,
    pub noise_floor_delta_db: f64,
    pub occupancy: f64,
    pub snr_drop_db: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelChange {
    pub id: Uuid,
    pub radio_id: Uuid,
    pub sensor_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub from_channel: u32,
    pub to_channel: u32,
    pub from_frequency_hz: u64,
    pub to_frequency_hz: u64,
    pub reason: String,
    pub state: ChannelChangeState,
    pub success: Option<bool>,
    pub rolled_back: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub level: AlertLevel,
    pub source: String,
    pub message: String,
    pub acknowledged: bool,
}
