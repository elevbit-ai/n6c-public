use anyhow::Result;
use specter_common::*;
use specter_detector::{DetectionConfig, InterferenceDetector};
use specter_dsp::{NoiseFloorEstimator, PSDConfig, PSDProcessor};
use specter_sdr::{create_driver, SDRConfig, SDRDriver};
use std::time::Duration;
use tracing::{error, info, warn};

pub struct SpecterSensor {
    config: SystemConfig,
    driver: Box<dyn SDRDriver>,
    psd: PSDProcessor,
    noise_floor: NoiseFloorEstimator,
    detector: InterferenceDetector,
    sequence: u64,
}

impl SpecterSensor {
    pub fn new(config: SystemConfig) -> Self {
        let driver = create_driver(&config.rf.device);
        let psd_config = PSDConfig {
            fft_size: config.rf.fft_size,
            sample_rate: config.rf.sample_rate,
            window: specter_dsp::WindowFunction::from_str(&config.rf.window_function),
            overlap: config.rf.overlap,
            averaging: 10,
        };
        let psd = PSDProcessor::new(psd_config).expect("Failed to create PSD processor");
        let noise_floor = NoiseFloorEstimator::new(200, 0.1);
        let detector = InterferenceDetector::new(DetectionConfig {
            confidence_threshold: config.policy.minimum_confidence,
            ..Default::default()
        });
        Self {
            config,
            driver,
            psd,
            noise_floor,
            detector,
            sequence: 0,
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing SPECTER-NET sensor...");
        let devices = self.driver.discover_devices().await?;
        if devices.is_empty() {
            error!("NO RF DEVICE AVAILABLE");
            warn!("No SDR hardware detected. Sensor will not capture real spectrum data.");
            warn!("Connect an SDR device (RTL-SDR, USRP, or SoapySDR-compatible) and restart.");
            return Ok(());
        }
        let device = &devices[0];
        info!("Found SDR device: {} ({})", device.name, device.driver);
        self.driver.open(&device.id).await?;
        self.driver
            .configure(&SDRConfig {
                sample_rate: self.config.rf.sample_rate,
                center_frequency_hz: self.config.rf.center_frequency_hz,
                gain: self.config.rf.gain,
                bandwidth_hz: self.config.rf.bandwidth_hz as u32,
                agc: false,
            })
            .await?;
        self.driver.start_streaming().await?;
        info!("Sensor initialized and streaming");
        Ok(())
    }

    pub async fn run_sweep(&mut self) -> Result<()> {
        let fft_size = self.config.rf.fft_size as usize;
        let sample_count = fft_size * 2;
        let buffer = self.driver.read_samples(sample_count).await?;
        let samples: Vec<f64> = buffer
            .samples
            .iter()
            .flat_map(|s| vec![s.i, s.q])
            .collect();
        let psd = self.psd.process(&samples)?;
        let psd_db = specter_dsp::magnitudes_to_db(&psd);
        let noise_floor = self.noise_floor.update(&psd_db)?;
        let mean_power: f64 = psd_db.iter().sum::<f64>() / psd_db.len() as f64;
        let peak_power = psd_db.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let snr = mean_power - noise_floor;
        let occupancy = psd_db
            .iter()
            .filter(|&&p| p > noise_floor + 10.0)
            .count() as f64
            / psd_db.len() as f64;
        let detection = self.detector.detect(noise_floor, occupancy, snr, 0.0, 1, 1);
        self.sequence += 1;
        let measurement = SpectrumMeasurement {
            sensor_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            center_frequency_hz: self.config.rf.center_frequency_hz,
            bandwidth_hz: self.config.rf.bandwidth_hz,
            sample_rate: self.config.rf.sample_rate,
            fft_size: self.config.rf.fft_size,
            noise_floor_dbm: noise_floor,
            peak_dbm: peak_power,
            mean_dbm: mean_power,
            snr_db: snr,
            occupancy,
            psd: psd_db,
        };
        if self.detector.should_trigger_event() {
            let event = RFEvent {
                id: uuid::Uuid::new_v4(),
                sensor_id: measurement.sensor_id,
                timestamp: measurement.timestamp,
                event_type: detection.classification.clone(),
                confidence: detection.confidence,
                center_frequency_hz: measurement.center_frequency_hz,
                bandwidth_hz: measurement.bandwidth_hz,
                duration_secs: self.detector.event_duration_secs(),
                noise_floor_delta_db: detection.noise_floor_delta_db,
                occupancy: detection.occupancy,
                snr_drop_db: detection.snr_db,
                description: format!(
                    "Event detected: {:?} (confidence: {:.2})",
                    detection.classification, detection.confidence
                ),
            };
            warn!("RF EVENT: {:?}", event);
        }
        Ok(())
    }

    pub async fn run_continuous(&mut self) -> Result<()> {
        info!("Starting continuous sweep mode");
        loop {
            if let Err(e) = self.run_sweep().await {
                error!("Sweep error: {}", e);
            }
            tokio::time::sleep(Duration::from_millis(self.config.rf.dwell_time_ms)).await;
        }
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down sensor...");
        self.driver.stop_streaming().await?;
        self.driver.close().await?;
        info!("Sensor shutdown complete");
        Ok(())
    }
}
