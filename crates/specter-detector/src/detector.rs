use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specter_common::RFClassification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionConfig {
    pub noise_floor_threshold_db: f64,
    pub occupancy_threshold: f64,
    pub snr_drop_threshold_db: f64,
    pub confidence_threshold: f64,
    pub min_event_duration_secs: f64,
    pub weights: DetectionWeights,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            noise_floor_threshold_db: 10.0,
            occupancy_threshold: 0.85,
            snr_drop_threshold_db: 10.0,
            confidence_threshold: 0.7,
            min_event_duration_secs: 5.0,
            weights: DetectionWeights::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionWeights {
    pub noise_floor_delta: f64,
    pub occupancy: f64,
    pub snr_drop: f64,
    pub packet_loss: f64,
    pub multi_sensor_correlation: f64,
}

impl Default for DetectionWeights {
    fn default() -> Self {
        Self {
            noise_floor_delta: 0.25,
            occupancy: 0.25,
            snr_drop: 0.20,
            packet_loss: 0.15,
            multi_sensor_correlation: 0.15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionState {
    pub classification: RFClassification,
    pub confidence: f64,
    pub noise_floor_delta_db: f64,
    pub occupancy: f64,
    pub snr_db: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct BaselineEntry {
    pub mean_noise_floor: f64,
    pub std_noise_floor: f64,
    pub mean_occupancy: f64,
    pub std_occupancy: f64,
    pub mean_snr: f64,
    pub std_snr: f64,
    pub hour_of_day: u32,
}

pub struct InterferenceDetector {
    config: DetectionConfig,
    baseline: Vec<BaselineEntry>,
    state: DetectionState,
    event_start: Option<DateTime<Utc>>,
    consecutive_detections: u32,
}

impl InterferenceDetector {
    pub fn new(config: DetectionConfig) -> Self {
        Self {
            config,
            baseline: Vec::new(),
            state: DetectionState {
                classification: RFClassification::Normal,
                confidence: 0.0,
                noise_floor_delta_db: 0.0,
                occupancy: 0.0,
                snr_db: 0.0,
                timestamp: Utc::now(),
            },
            event_start: None,
            consecutive_detections: 0,
        }
    }

    pub fn update_baseline(&mut self, entry: BaselineEntry) {
        if self.baseline.len() > 1000 {
            self.baseline.remove(0);
        }
        self.baseline.push(entry);
    }

    pub fn detect(
        &mut self,
        noise_floor_db: f64,
        occupancy: f64,
        snr_db: f64,
        packet_loss_rate: f64,
        multi_sensor_count: u32,
        total_sensors: u32,
    ) -> DetectionState {
        let baseline = self.get_current_baseline();
        let noise_delta =
            (noise_floor_db - baseline.mean_noise_floor).abs() / baseline.std_noise_floor.max(1.0);
        let occ_delta =
            (occupancy - baseline.mean_occupancy).abs() / baseline.std_occupancy.max(0.01);
        let snr_delta = (snr_db - baseline.mean_snr).abs() / baseline.std_snr.max(1.0);

        let w = &self.config.weights;
        let noise_score = self.normalize(noise_delta, self.config.noise_floor_threshold_db);
        let occ_score = self.normalize(occ_delta, 1.0 - self.config.occupancy_threshold);
        let snr_score = self.normalize(snr_delta, self.config.snr_drop_threshold_db);
        let pkt_score = packet_loss_rate.min(1.0);
        let corr_score = if total_sensors > 0 {
            multi_sensor_count as f64 / total_sensors as f64
        } else {
            0.0
        };

        let confidence = (w.noise_floor_delta * noise_score
            + w.occupancy * occ_score
            + w.snr_drop * snr_score
            + w.packet_loss * pkt_score
            + w.multi_sensor_correlation * corr_score)
            .clamp(0.0, 1.0);

        let classification = self.classify(confidence, occupancy, noise_delta, snr_delta);
        let now = Utc::now();

        match classification {
            RFClassification::Normal => {
                self.event_start = None;
                self.consecutive_detections = 0;
            }
            _ => {
                if self.event_start.is_none() {
                    self.event_start = Some(now);
                }
                self.consecutive_detections += 1;
            }
        }

        self.state = DetectionState {
            classification,
            confidence,
            noise_floor_delta_db: noise_delta,
            occupancy,
            snr_db: snr_db,
            timestamp: now,
        };
        self.state.clone()
    }

    fn classify(
        &self,
        confidence: f64,
        occupancy: f64,
        noise_delta: f64,
        snr_delta: f64,
    ) -> RFClassification {
        if confidence < 0.3 {
            RFClassification::Normal
        } else if confidence < 0.5 {
            RFClassification::ElevatedNoise
        } else if confidence < 0.7 {
            RFClassification::InterferenceSuspected
        } else if occupancy > 0.95 && noise_delta > 2.0 && snr_delta > 2.0 {
            RFClassification::JammingSuspected
        } else if confidence > 0.8 {
            RFClassification::Degraded
        } else {
            RFClassification::InterferenceSuspected
        }
    }

    fn normalize(&self, delta: f64, threshold: f64) -> f64 {
        (delta / threshold).clamp(0.0, 1.0)
    }

    fn get_current_baseline(&self) -> BaselineEntry {
        if self.baseline.is_empty() {
            return BaselineEntry {
                mean_noise_floor: -80.0,
                std_noise_floor: 5.0,
                mean_occupancy: 0.1,
                std_occupancy: 0.05,
                mean_snr: 30.0,
                std_snr: 5.0,
                hour_of_day: 0,
            };
        }
        let now = Utc::now();
        let hour = now.format("%H").to_string().parse::<u32>().unwrap_or(0);
        let matching: Vec<&BaselineEntry> = self
            .baseline
            .iter()
            .filter(|e| (e.hour_of_day as i32 - hour as i32).abs() <= 1)
            .collect();
        if matching.is_empty() {
            return self.baseline.last().cloned().unwrap();
        }
        let n = matching.len() as f64;
        BaselineEntry {
            mean_noise_floor: matching.iter().map(|e| e.mean_noise_floor).sum::<f64>() / n,
            std_noise_floor: matching.iter().map(|e| e.std_noise_floor).sum::<f64>() / n,
            mean_occupancy: matching.iter().map(|e| e.mean_occupancy).sum::<f64>() / n,
            std_occupancy: matching.iter().map(|e| e.std_occupancy).sum::<f64>() / n,
            mean_snr: matching.iter().map(|e| e.mean_snr).sum::<f64>() / n,
            std_snr: matching.iter().map(|e| e.std_snr).sum::<f64>() / n,
            hour_of_day: hour,
        }
    }

    pub fn state(&self) -> &DetectionState {
        &self.state
    }

    pub fn should_trigger_event(&self) -> bool {
        self.state.confidence >= self.config.confidence_threshold
            && self.consecutive_detections >= 3
    }

    pub fn event_duration_secs(&self) -> f64 {
        self.event_start
            .map(|start| (Utc::now() - start).num_milliseconds() as f64 / 1000.0)
            .unwrap_or(0.0)
    }

    pub fn config(&self) -> &DetectionConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_normal() {
        let mut det = InterferenceDetector::new(DetectionConfig::default());
        let state = det.detect(-80.0, 0.1, 30.0, 0.0, 0, 5);
        assert_eq!(state.classification, RFClassification::Normal);
        assert!(state.confidence < 0.5);
    }

    #[test]
    fn test_detector_degraded() {
        let mut det = InterferenceDetector::new(DetectionConfig::default());
        for _ in 0..5 {
            det.detect(-50.0, 0.98, 2.0, 0.3, 4, 5);
        }
        let state = det.detect(-50.0, 0.98, 2.0, 0.3, 4, 5);
        assert!(state.confidence > 0.5);
    }
}
