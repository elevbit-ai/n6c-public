use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDRDevice {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub serial: Option<String>,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDRConfig {
    pub sample_rate: u32,
    pub center_frequency_hz: u64,
    pub gain: f64,
    pub bandwidth_hz: u32,
    pub agc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IQSample {
    pub i: f64,
    pub q: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IQBuffer {
    pub samples: Vec<IQSample>,
    pub sample_rate: u32,
    pub center_frequency_hz: u64,
    pub timestamp_samples: u64,
}
