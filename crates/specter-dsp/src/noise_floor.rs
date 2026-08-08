use crate::DSPResult;

#[derive(Debug, Clone)]
pub struct NoiseFloorEstimator {
    buffer: Vec<f64>,
    capacity: usize,
    percentile: f64,
}

impl NoiseFloorEstimator {
    pub fn new(capacity: usize, percentile: f64) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
            percentile,
        }
    }

    pub fn update(&mut self, psd_db: &[f64]) -> DSPResult<f64> {
        if psd_db.is_empty() {
            return Err(crate::DSPError::ProcessingError(
                "Empty PSD input".to_string(),
            ));
        }
        let mean: f64 = psd_db.iter().sum::<f64>() / psd_db.len() as f64;
        self.buffer.push(mean);
        if self.buffer.len() > self.capacity {
            self.buffer.remove(0);
        }
        Ok(self.estimate())
    }

    pub fn estimate(&self) -> f64 {
        if self.buffer.is_empty() {
            return -100.0;
        }
        let mut sorted = self.buffer.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let idx = (sorted.len() as f64 * self.percentile) as usize;
        sorted[idx.min(sorted.len() - 1)]
    }

    pub fn estimate_robust(&self) -> f64 {
        if self.buffer.is_empty() {
            return -100.0;
        }
        let mut sorted = self.buffer.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let q1_idx = sorted.len() / 4;
        let q3_idx = (sorted.len() * 3) / 4;
        let q1 = sorted[q1_idx];
        let q3 = sorted[q3_idx.min(sorted.len() - 1)];
        let iqr = q3 - q1;
        let lower = q1 - 1.5 * iqr;
        let upper = q3 + 1.5 * iqr;
        let filtered: Vec<f64> = sorted
            .iter()
            .filter(|v| **v >= lower && **v <= upper)
            .cloned()
            .collect();
        if filtered.is_empty() {
            return q1;
        }
        filtered.iter().sum::<f64>() / filtered.len() as f64
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

pub struct MultiBandNoiseFloor {
    estimators: Vec<NoiseFloorEstimator>,
    band_count: usize,
}

impl MultiBandNoiseFloor {
    pub fn new(band_count: usize, capacity: usize, percentile: f64) -> Self {
        let estimators = (0..band_count)
            .map(|_| NoiseFloorEstimator::new(capacity, percentile))
            .collect();
        Self {
            estimators,
            band_count,
        }
    }

    pub fn update(&mut self, psd_db: &[f64]) -> DSPResult<Vec<f64>> {
        if psd_db.len() < self.band_count {
            return Err(crate::DSPError::ProcessingError(
                "PSD bins fewer than bands".to_string(),
            ));
        }
        let bin_per_band = psd_db.len() / self.band_count;
        let mut floors = Vec::with_capacity(self.band_count);
        for (i, estimator) in self.estimators.iter_mut().enumerate() {
            let start = i * bin_per_band;
            let end = start + bin_per_band;
            let band_psd = &psd_db[start..end];
            let floor = estimator.update(band_psd)?;
            floors.push(floor);
        }
        Ok(floors)
    }

    pub fn estimate(&self) -> Vec<f64> {
        self.estimators.iter().map(|e| e.estimate()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_floor_estimator() {
        let mut estimator = NoiseFloorEstimator::new(100, 0.1);
        for _ in 0..50 {
            let psd: Vec<f64> = (0..100).map(|_| -80.0 + rand_noise()).collect();
            estimator.update(&psd).unwrap();
        }
        let floor = estimator.estimate();
        assert!(floor > -120.0 && floor < -40.0);
    }

    fn rand_noise() -> f64 {
        (std::f64::consts::PI * 42.0).sin() * 5.0
    }
}
