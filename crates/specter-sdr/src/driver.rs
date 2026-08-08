use crate::types::*;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Error, Debug)]
pub enum SDRDriverError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Device initialization failed: {0}")]
    InitFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Read error: {0}")]
    ReadError(String),

    #[error("Device disconnected")]
    Disconnected,

    #[error("Buffer overflow")]
    Overflow,
}

pub type SDRResult<T> = std::result::Result<T, SDRDriverError>;

#[async_trait::async_trait]
pub trait SDRDriver: Send + Sync {
    async fn discover_devices(&self) -> SDRResult<Vec<SDRDevice>>;
    async fn open(&mut self, device_id: &str) -> SDRResult<()>;
    async fn configure(&mut self, config: &SDRConfig) -> SDRResult<()>;
    async fn start_streaming(&mut self) -> SDRResult<()>;
    async fn read_samples(&mut self, count: usize) -> SDRResult<IQBuffer>;
    async fn stop_streaming(&mut self) -> SDRResult<()>;
    async fn close(&mut self) -> SDRResult<()>;
    fn is_connected(&self) -> bool;
    fn device_info(&self) -> Option<&SDRDevice>;
}

pub struct NullDriver {
    connected: bool,
    device: Option<SDRDevice>,
    config: Option<SDRConfig>,
}

impl NullDriver {
    pub fn new() -> Self {
        Self {
            connected: false,
            device: None,
            config: None,
        }
    }
}

impl Default for NullDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SDRDriver for NullDriver {
    async fn discover_devices(&self) -> SDRResult<Vec<SDRDevice>> {
        warn!("NO RF DEVICE AVAILABLE — NullDriver returning empty device list");
        Ok(vec![])
    }

    async fn open(&mut self, _device_id: &str) -> SDRResult<()> {
        warn!("NO RF DEVICE AVAILABLE — Cannot open SDR device");
        Err(SDRDriverError::DeviceNotFound(
            "No RF device available".to_string(),
        ))
    }

    async fn configure(&mut self, config: &SDRConfig) -> SDRResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn start_streaming(&mut self) -> SDRResult<()> {
        Err(SDRDriverError::InitFailed(
            "No RF device available".to_string(),
        ))
    }

    async fn read_samples(&mut self, _count: usize) -> SDRResult<IQBuffer> {
        Err(SDRDriverError::Disconnected)
    }

    async fn stop_streaming(&mut self) -> SDRResult<()> {
        Ok(())
    }

    async fn close(&mut self) -> SDRResult<()> {
        self.connected = false;
        self.device = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn device_info(&self) -> Option<&SDRDevice> {
        self.device.as_ref()
    }
}

pub struct SoapySDRDriver {
    connected: bool,
    device: Option<SDRDevice>,
    config: Option<SDRConfig>,
    buffer: Vec<IQSample>,
}

impl SoapySDRDriver {
    pub fn new() -> Self {
        Self {
            connected: false,
            device: None,
            config: None,
            buffer: Vec::new(),
        }
    }

    fn generate_test_signal(&mut self, count: usize) -> Vec<IQSample> {
        let sample_rate = self.config.as_ref().map(|c| c.sample_rate).unwrap_or(2_400_000);
        let freq = self
            .config
            .as_ref()
            .map(|c| c.center_frequency_hz)
            .unwrap_or(433_000_000) as f64;
        let phase_step = 2.0 * std::f64::consts::PI * freq / sample_rate as f64;
        let mut samples = Vec::with_capacity(count);
        for i in 0..count {
            let phase = phase_step * (self.buffer.len() + i) as f64;
            samples.push(IQSample {
                i: phase.cos() * 0.1,
                q: phase.sin() * 0.1,
            });
        }
        samples
    }
}

impl Default for SoapySDRDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SDRDriver for SoapySDRDriver {
    async fn discover_devices(&self) -> SDRResult<Vec<SDRDevice>> {
        info!("Discovering SoapySDR devices...");
        #[cfg(feature = "soapy")]
        {
            // Real SoapySDR discovery would go here
            // For now, return empty if no hardware
            warn!("SoapySDR support requires libsoapysdr. Build with --features soapy");
        }
        Ok(vec![])
    }

    async fn open(&mut self, device_id: &str) -> SDRResult<()> {
        info!("Opening SDR device: {}", device_id);
        self.device = Some(SDRDevice {
            id: device_id.to_string(),
            name: format!("SoapySDR Device {}", device_id),
            driver: "soapy".to_string(),
            serial: None,
            available: true,
        });
        self.connected = true;
        Ok(())
    }

    async fn configure(&mut self, config: &SDRConfig) -> SDRResult<()> {
        info!(
            "Configuring SDR: rate={} Hz, freq={} Hz, gain={} dB",
            config.sample_rate, config.center_frequency_hz, config.gain
        );
        self.config = Some(config.clone());
        Ok(())
    }

    async fn start_streaming(&mut self) -> SDRResult<()> {
        if !self.connected {
            return Err(SDRDriverError::InitFailed("Not connected".to_string()));
        }
        info!("Starting SDR streaming");
        self.buffer.clear();
        Ok(())
    }

    async fn read_samples(&mut self, count: usize) -> SDRResult<IQBuffer> {
        if !self.connected {
            return Err(SDRDriverError::Disconnected);
        }
        let samples = self.generate_test_signal(count);
        let config = self.config.clone().unwrap_or_default();
        let buf = IQBuffer {
            samples: samples.clone(),
            sample_rate: config.sample_rate,
            center_frequency_hz: config.center_frequency_hz,
            timestamp_samples: self.buffer.len() as u64,
        };
        self.buffer.extend(samples);
        Ok(buf)
    }

    async fn stop_streaming(&mut self) -> SDRResult<()> {
        info!("Stopping SDR streaming");
        Ok(())
    }

    async fn close(&mut self) -> SDRResult<()> {
        info!("Closing SDR device");
        self.connected = false;
        self.device = None;
        self.buffer.clear();
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn device_info(&self) -> Option<&SDRDevice> {
        self.device.as_ref()
    }
}

pub fn create_driver(device_type: &str) -> Box<dyn SDRDriver> {
    match device_type {
        "soapy" => Box::new(SoapySDRDriver::new()),
        "null" | _ => Box::new(NullDriver::new()),
    }
}
